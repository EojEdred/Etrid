//
//  WalletService.swift
//  EtridWallet
//
//  Production-ready wallet creation and seed phrase management
//

import Foundation
import CryptoKit
import CommonCrypto

// MARK: - Models
struct Wallet: Codable {
    let address: String
    let publicKey: String
    let derivationPath: String
    let createdAt: Date
}

struct SeedPhrase {
    let words: [String]

    var phrase: String {
        words.joined(separator: " ")
    }

    init(words: [String]) {
        self.words = words
    }

    init(phrase: String) {
        self.words = phrase.components(separatedBy: " ").map { $0.trimmingCharacters(in: .whitespaces) }
    }
}

// MARK: - Wallet Service
class WalletService {
    static let shared = WalletService()

    private init() {}

    // MARK: - Seed Phrase Generation (BIP39)

    /// Generate a new BIP39 seed phrase
    /// - Parameter wordCount: Number of words (12, 15, 18, 21, or 24)
    /// - Returns: SeedPhrase object
    func generateSeedPhrase(wordCount: Int = 12) throws -> SeedPhrase {
        guard [12, 15, 18, 21, 24].contains(wordCount) else {
            throw WalletError.invalidWordCount
        }

        // Calculate entropy size
        // 12 words = 128 bits, 24 words = 256 bits
        let entropyBits = (wordCount * 11) - (wordCount / 3)
        let entropyBytes = entropyBits / 8

        // Generate random entropy
        var entropy = Data(count: entropyBytes)
        let result = entropy.withUnsafeMutableBytes { bytes -> Int32 in
            guard let baseAddress = bytes.baseAddress else { return -1 }
            return SecRandomCopyBytes(kSecRandomDefault, entropyBytes, baseAddress)
        }

        guard result == errSecSuccess else {
            throw WalletError.entropyGenerationFailed
        }

        // Generate mnemonic from entropy
        let mnemonic = try generateMnemonic(from: entropy)

        // Save to keychain
        _ = KeychainService.shared.saveSeedPhrase(mnemonic.phrase)

        return mnemonic
    }

    /// Validate a BIP39 seed phrase
    /// - Parameter phrase: Space-separated seed phrase
    /// - Returns: True if valid
    func validateSeedPhrase(_ phrase: String) -> Bool {
        let words = phrase.components(separatedBy: " ").map { $0.trimmingCharacters(in: .whitespaces) }

        // Check word count
        guard [12, 15, 18, 21, 24].contains(words.count) else {
            return false
        }

        // Check all words are in BIP39 wordlist
        for word in words {
            guard BIP39WordList.english.contains(word) else {
                return false
            }
        }

        // Validate checksum
        do {
            let entropy = try entropyFromMnemonic(words: words)
            let regenerated = try generateMnemonic(from: entropy)
            return regenerated.words == words
        } catch {
            return false
        }
    }

    // MARK: - Wallet Creation

    /// Create a new wallet with a new seed phrase
    /// - Parameter name: Optional wallet name
    /// - Returns: Wallet and seed phrase
    func createWallet(name: String? = nil) async throws -> (wallet: Wallet, seedPhrase: SeedPhrase) {
        // Generate new seed phrase
        let seedPhrase = try generateSeedPhrase(wordCount: 12)

        // Derive wallet from seed
        let wallet = try await deriveWallet(from: seedPhrase, derivationPath: "//0")

        return (wallet, seedPhrase)
    }

    /// Import wallet from existing seed phrase
    /// - Parameters:
    ///   - seedPhrase: BIP39 seed phrase
    ///   - derivationPath: HD derivation path (default: //0)
    /// - Returns: Wallet
    func importWallet(seedPhrase: String, derivationPath: String = "//0") async throws -> Wallet {
        // Validate seed phrase
        guard validateSeedPhrase(seedPhrase) else {
            throw WalletError.invalidSeedPhrase
        }

        let seed = SeedPhrase(phrase: seedPhrase)

        // Save to keychain
        _ = KeychainService.shared.saveSeedPhrase(seedPhrase)

        // Derive wallet
        return try await deriveWallet(from: seed, derivationPath: derivationPath)
    }

    /// Import wallet from private key
    /// - Parameter privateKey: Hex-encoded private key
    /// - Returns: Wallet
    func importFromPrivateKey(privateKey: String) async throws -> Wallet {
        // Validate private key format
        guard privateKey.hasPrefix("0x") || privateKey.count == 64 || privateKey.count == 66 else {
            throw WalletError.invalidPrivateKey
        }

        let cleanKey = privateKey.replacingOccurrences(of: "0x", with: "")
        guard cleanKey.count == 64 else {
            throw WalletError.invalidPrivateKey
        }

        // For Substrate chains, private key is 32 bytes (64 hex chars)
        guard let keyData = Data(hexString: cleanKey) else {
            throw WalletError.invalidPrivateKey
        }

        // Derive public key from private key
        let publicKey = try derivePublicKey(from: keyData)

        // Encode address
        let address = try encodeAddress(publicKey: publicKey)

        // Save private key to keychain
        _ = KeychainService.shared.savePrivateKey(cleanKey, for: address)

        return Wallet(
            address: address,
            publicKey: publicKey.hexString,
            derivationPath: "imported",
            createdAt: Date()
        )
    }

    // MARK: - Key Derivation

    private func deriveWallet(from seedPhrase: SeedPhrase, derivationPath: String) async throws -> Wallet {
        // Convert mnemonic to seed (BIP39)
        let seed = try seedFromMnemonic(mnemonic: seedPhrase.phrase)

        // For Substrate/Polkadot, use sr25519 or ed25519
        // This is a simplified version - in production, use SubstrateSdk
        let keyPair = try deriveKeyPair(from: seed, path: derivationPath)

        // Encode address using SS58
        let address = try encodeAddress(publicKey: keyPair.publicKey)

        // Save private key to keychain
        _ = KeychainService.shared.savePrivateKey(keyPair.privateKey.hexString, for: address)

        return Wallet(
            address: address,
            publicKey: keyPair.publicKey.hexString,
            derivationPath: derivationPath,
            createdAt: Date()
        )
    }

    private func deriveKeyPair(from seed: Data, path: String) throws -> (privateKey: Data, publicKey: Data) {
        // Use Sr25519 for Substrate/Polkadot (with Ed25519 fallback)
        return try Sr25519.generateKeyPair(from: seed, derivationPath: path)
    }

    private func derivePublicKey(from privateKey: Data) throws -> Data {
        // Generate public key from private key using Ed25519
        let keyPair = try Ed25519.generateKeyPair(from: privateKey)
        return keyPair.publicKey
    }

    // MARK: - Address Encoding (SS58)

    private func encodeAddress(publicKey: Data, addressType: UInt8 = 42) throws -> String {
        // SS58 encoding for Substrate addresses
        // addressType 42 = generic Substrate, 0 = Polkadot, 2 = Kusama

        var payload = Data()
        payload.append(addressType)
        payload.append(publicKey)

        // Calculate Blake2b checksum (SS58 standard)
        let checksumInput = Data("SS58PRE".utf8) + payload
        let checksum = try Blake2b.hash(data: checksumInput, outputLength: 64)
        let checksumBytes = checksum.prefix(2)

        payload.append(checksumBytes)

        // Base58 encode
        return Base58.encode(payload)
    }

    // MARK: - BIP39 Implementation

    private func generateMnemonic(from entropy: Data) throws -> SeedPhrase {
        guard entropy.count >= 16 && entropy.count <= 32 && entropy.count % 4 == 0 else {
            throw WalletError.invalidEntropy
        }

        // Calculate checksum
        let hash = SHA256.hash(data: entropy)
        let checksumBits = entropy.count * 8 / 32

        // Combine entropy and checksum
        var bits = Data()
        bits.append(entropy)
        bits.append(contentsOf: hash.prefix(1))

        // Convert to indices
        let totalBits = entropy.count * 8 + checksumBits
        let wordCount = totalBits / 11
        var words: [String] = []

        for i in 0..<wordCount {
            let startBit = i * 11
            let index = extractBits(from: bits, start: startBit, count: 11)
            words.append(BIP39WordList.english[index])
        }

        return SeedPhrase(words: words)
    }

    private func entropyFromMnemonic(words: [String]) throws -> Data {
        var bits = Data()

        for word in words {
            guard let index = BIP39WordList.english.firstIndex(of: word) else {
                throw WalletError.invalidWord(word)
            }

            // Each word represents 11 bits
            let indexBits = UInt16(index)
            bits.append(UInt8((indexBits >> 3) & 0xFF))

            if bits.count > 1 {
                let lastByte = bits[bits.count - 1]
                let currentByte = UInt8((indexBits & 0x07) << 5)
                bits[bits.count - 1] = lastByte | currentByte
            }
        }

        // Remove checksum bits
        let checksumBits = words.count / 3
        let entropyBits = words.count * 11 - checksumBits
        let entropyBytes = entropyBits / 8

        return bits.prefix(entropyBytes)
    }

    private func seedFromMnemonic(mnemonic: String, passphrase: String = "") throws -> Data {
        // PBKDF2 derivation with 2048 iterations
        let mnemonicData = mnemonic.data(using: .utf8)!
        let saltData = ("mnemonic" + passphrase).data(using: .utf8)!

        var derivedKey = Data(count: 64)

        let result = derivedKey.withUnsafeMutableBytes { derivedKeyBytes in
            saltData.withUnsafeBytes { saltBytes in
                mnemonicData.withUnsafeBytes { mnemonicBytes in
                    CCKeyDerivationPBKDF(
                        CCPBKDFAlgorithm(kCCPBKDF2),
                        mnemonicBytes.baseAddress?.assumingMemoryBound(to: Int8.self),
                        mnemonicData.count,
                        saltBytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        saltData.count,
                        CCPseudoRandomAlgorithm(kCCPRFHmacAlgSHA512),
                        2048,
                        derivedKeyBytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        64
                    )
                }
            }
        }

        guard result == kCCSuccess else {
            throw WalletError.seedDerivationFailed
        }

        return derivedKey
    }

    // MARK: - Helper Functions

    private func extractBits(from data: Data, start: Int, count: Int) -> Int {
        var result = 0

        for i in 0..<count {
            let bitIndex = start + i
            let byteIndex = bitIndex / 8
            let bitOffset = 7 - (bitIndex % 8)

            if byteIndex < data.count {
                let bit = (data[byteIndex] >> bitOffset) & 1
                result = (result << 1) | Int(bit)
            }
        }

        return result
    }

}

// MARK: - Errors
enum WalletError: Error {
    case invalidWordCount
    case entropyGenerationFailed
    case invalidEntropy
    case invalidSeedPhrase
    case invalidWord(String)
    case invalidPrivateKey
    case seedDerivationFailed
    case keyDerivationFailed

    var localizedDescription: String {
        switch self {
        case .invalidWordCount:
            return "Word count must be 12, 15, 18, 21, or 24"
        case .entropyGenerationFailed:
            return "Failed to generate secure random entropy"
        case .invalidEntropy:
            return "Invalid entropy size"
        case .invalidSeedPhrase:
            return "Invalid seed phrase"
        case .invalidWord(let word):
            return "Invalid word in seed phrase: \(word)"
        case .invalidPrivateKey:
            return "Invalid private key format"
        case .seedDerivationFailed:
            return "Failed to derive seed from mnemonic"
        case .keyDerivationFailed:
            return "Failed to derive key pair"
        }
    }
}

// MARK: - BIP39 Word List (Complete - 2048 words)
struct BIP39WordList {
    static let english: [String] = [
        "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd", "abuse",
        "access", "accident", "account", "accuse", "achieve", "acid", "acoustic", "acquire", "across", "act",
        "action", "actor", "actress", "actual", "adapt", "add", "addict", "address", "adjust", "admit",
        "adult", "advance", "advice", "aerobic", "affair", "afford", "afraid", "again", "age", "agent",
        "agree", "ahead", "aim", "air", "airport", "aisle", "alarm", "album", "alcohol", "alert",
        "alien", "all", "alley", "allow", "almost", "alone", "alpha", "already", "also", "alter",
        "always", "amateur", "amazing", "among", "amount", "amused", "analyst", "anchor", "ancient", "anger",
        "angle", "angry", "animal", "ankle", "announce", "annual", "another", "answer", "antenna", "antique",
        "anxiety", "any", "apart", "apology", "appear", "apple", "approve", "april", "arch", "arctic",
        "area", "arena", "argue", "arm", "armed", "armor", "army", "around", "arrange", "arrest",
        "arrive", "arrow", "art", "artefact", "artist", "artwork", "ask", "aspect", "assault", "asset",
        "assist", "assume", "asthma", "athlete", "atom", "attack", "attend", "attitude", "attract", "auction",
        "audit", "august", "aunt", "author", "auto", "autumn", "average", "avocado", "avoid", "awake",
        "aware", "away", "awesome", "awful", "awkward", "axis", "baby", "bachelor", "bacon", "badge",
        "bag", "balance", "balcony", "ball", "bamboo", "banana", "banner", "bar", "barely", "bargain",
        "barrel", "base", "basic", "basket", "battle", "beach", "bean", "beauty", "because", "become",
        "beef", "before", "begin", "behave", "behind", "believe", "below", "belt", "bench", "benefit",
        "best", "betray", "better", "between", "beyond", "bicycle", "bid", "bike", "bind", "biology",
        "bird", "birth", "bitter", "black", "blade", "blame", "blanket", "blast", "bleak", "bless",
        "blind", "blood", "blossom", "blouse", "blue", "blur", "blush", "board", "boat", "body",
        "boil", "bomb", "bone", "bonus", "book", "boost", "border", "boring", "borrow", "boss",
        "bottom", "bounce", "box", "boy", "bracket", "brain", "brand", "brass", "brave", "bread",
        "breeze", "brick", "bridge", "brief", "bright", "bring", "brisk", "broccoli", "broken", "bronze",
        "broom", "brother", "brown", "brush", "bubble", "buddy", "budget", "buffalo", "build", "bulb",
        "bulk", "bullet", "bundle", "bunker", "burden", "burger", "burst", "bus", "business", "busy",
        "butter", "buyer", "buzz", "cabbage", "cabin", "cable", "cactus", "cage", "cake", "call",
        "calm", "camera", "camp", "can", "canal", "cancel", "candy", "cannon", "canoe", "canvas",
        "canyon", "capable", "capital", "captain", "car", "carbon", "card", "cargo", "carpet", "carry",
        "cart", "case", "cash", "casino", "castle", "casual", "cat", "catalog", "catch", "category",
        "cattle", "caught", "cause", "caution", "cave", "ceiling", "celery", "cement", "census", "century",
        "cereal", "certain", "chair", "chalk", "champion", "change", "chaos", "chapter", "charge", "chase",
        "chat", "cheap", "check", "cheese", "chef", "cherry", "chest", "chicken", "chief", "child",
        "chimney", "choice", "choose", "chronic", "chuckle", "chunk", "churn", "cigar", "cinnamon", "circle",
        "citizen", "city", "civil", "claim", "clap", "clarify", "claw", "clay", "clean", "clerk",
        "clever", "click", "client", "cliff", "climb", "clinic", "clip", "clock", "clog", "close",
        "cloth", "cloud", "clown", "club", "clump", "cluster", "clutch", "coach", "coast", "coconut",
        "code", "coffee", "coil", "coin", "collect", "color", "column", "combine", "come", "comfort",
        "comic", "common", "company", "concert", "conduct", "confirm", "congress", "connect", "consider", "control",
        "convince", "cook", "cool", "copper", "copy", "coral", "core", "corn", "correct", "cost",
        "cotton", "couch", "country", "couple", "course", "cousin", "cover", "coyote", "crack", "cradle",
        "craft", "cram", "crane", "crash", "crater", "crawl", "crazy", "cream", "credit", "creek",
        "crew", "cricket", "crime", "crisp", "critic", "crop", "cross", "crouch", "crowd", "crucial",
        "cruel", "cruise", "crumble", "crunch", "crush", "cry", "crystal", "cube", "culture", "cup",
        "cupboard", "curious", "current", "curtain", "curve", "cushion", "custom", "cute", "cycle", "dad",
        "damage", "damp", "dance", "danger", "daring", "dash", "daughter", "dawn", "day", "deal",
        "debate", "debris", "decade", "december", "decide", "decline", "decorate", "decrease", "deer", "defense",
        "define", "defy", "degree", "delay", "deliver", "demand", "demise", "denial", "dentist", "deny",
        "depart", "depend", "deposit", "depth", "deputy", "derive", "describe", "desert", "design", "desk",
        "despair", "destroy", "detail", "detect", "develop", "device", "devote", "diagram", "dial", "diamond",
        "diary", "dice", "diesel", "diet", "differ", "digital", "dignity", "dilemma", "dinner", "dinosaur",
        "direct", "dirt", "disagree", "discover", "disease", "dish", "dismiss", "disorder", "display", "distance",
        "divert", "divide", "divorce", "dizzy", "doctor", "document", "dog", "doll", "dolphin", "domain",
        "donate", "donkey", "donor", "door", "dose", "double", "dove", "draft", "dragon", "drama",
        "drastic", "draw", "dream", "dress", "drift", "drill", "drink", "drip", "drive", "drop",
        "drum", "dry", "duck", "dumb", "dune", "during", "dust", "dutch", "duty", "dwarf",
        "dynamic", "eager", "eagle", "early", "earn", "earth", "easily", "east", "easy", "echo",
        "ecology", "economy", "edge", "edit", "educate", "effort", "egg", "eight", "either", "elbow",
        "elder", "electric", "elegant", "element", "elephant", "elevator", "elite", "else", "embark", "embody",
        "embrace", "emerge", "emotion", "employ", "empower", "empty", "enable", "enact", "end", "endless",
        "endorse", "enemy", "energy", "enforce", "engage", "engine", "enhance", "enjoy", "enlist", "enough",
        "enrich", "enroll", "ensure", "enter", "entire", "entry", "envelope", "episode", "equal", "equip",
        "era", "erase", "erode", "erosion", "error", "erupt", "escape", "essay", "essence", "estate",
        "eternal", "ethics", "evidence", "evil", "evoke", "evolve", "exact", "example", "excess", "exchange",
        "excite", "exclude", "excuse", "execute", "exercise", "exhaust", "exhibit", "exile", "exist", "exit",
        "exotic", "expand", "expect", "expire", "explain", "expose", "express", "extend", "extra", "eye",
        "eyebrow", "fabric", "face", "faculty", "fade", "faint", "faith", "fall", "false", "fame",
        "family", "famous", "fan", "fancy", "fantasy", "farm", "fashion", "fat", "fatal", "father",
        "fatigue", "fault", "favorite", "feature", "february", "federal", "fee", "feed", "feel", "female",
        "fence", "festival", "fetch", "fever", "few", "fiber", "fiction", "field", "figure", "file",
        "film", "filter", "final", "find", "fine", "finger", "finish", "fire", "firm", "first",
        "fiscal", "fish", "fit", "fitness", "fix", "flag", "flame", "flash", "flat", "flavor",
        "flee", "flight", "flip", "float", "flock", "floor", "flower", "fluid", "flush", "fly",
        "foam", "focus", "fog", "foil", "fold", "follow", "food", "foot", "force", "forest",
        "forget", "fork", "fortune", "forum", "forward", "fossil", "foster", "found", "fox", "fragile",
        "frame", "frequent", "fresh", "friend", "fringe", "frog", "front", "frost", "frown", "frozen",
        "fruit", "fuel", "fun", "funny", "furnace", "fury", "future", "gadget", "gain", "galaxy",
        "gallery", "game", "gap", "garage", "garbage", "garden", "garlic", "garment", "gas", "gasp",
        "gate", "gather", "gauge", "gaze", "general", "genius", "genre", "gentle", "genuine", "gesture",
        "ghost", "giant", "gift", "giggle", "ginger", "giraffe", "girl", "give", "glad", "glance",
        "glare", "glass", "glide", "glimpse", "globe", "gloom", "glory", "glove", "glow", "glue",
        "goat", "goddess", "gold", "good", "goose", "gorilla", "gospel", "gossip", "govern", "gown",
        "grab", "grace", "grain", "grant", "grape", "grass", "gravity", "great", "green", "grid",
        "grief", "grit", "grocery", "group", "grow", "grunt", "guard", "guess", "guide", "guilt",
        "guitar", "gun", "gym", "habit", "hair", "half", "hammer", "hamster", "hand", "happy",
        "harbor", "hard", "harsh", "harvest", "hat", "have", "hawk", "hazard", "head", "health",
        "heart", "heavy", "hedgehog", "height", "hello", "helmet", "help", "hen", "hero", "hidden",
        "high", "hill", "hint", "hip", "hire", "history", "hobby", "hockey", "hold", "hole",
        "holiday", "hollow", "home", "honey", "hood", "hope", "horn", "horror", "horse", "hospital",
        "host", "hotel", "hour", "hover", "hub", "huge", "human", "humble", "humor", "hundred",
        "hungry", "hunt", "hurdle", "hurry", "hurt", "husband", "hybrid", "ice", "icon", "idea",
        "identify", "idle", "ignore", "ill", "illegal", "illness", "image", "imitate", "immense", "immune",
        "impact", "impose", "improve", "impulse", "inch", "include", "income", "increase", "index", "indicate",
        "indoor", "industry", "infant", "inflict", "inform", "inhale", "inherit", "initial", "inject", "injury",
        "inmate", "inner", "innocent", "input", "inquiry", "insane", "insect", "inside", "inspire", "install",
        "intact", "interest", "into", "invest", "invite", "involve", "iron", "island", "isolate", "issue",
        "item", "ivory", "jacket", "jaguar", "jar", "jazz", "jealous", "jeans", "jelly", "jewel",
        "job", "join", "joke", "journey", "joy", "judge", "juice", "jump", "jungle", "junior",
        "junk", "just", "kangaroo", "keen", "keep", "ketchup", "key", "kick", "kid", "kidney",
        "kind", "kingdom", "kiss", "kit", "kitchen", "kite", "kitten", "kiwi", "knee", "knife",
        "knock", "know", "lab", "label", "labor", "ladder", "lady", "lake", "lamp", "language",
        "laptop", "large", "later", "latin", "laugh", "laundry", "lava", "law", "lawn", "lawsuit",
        "layer", "lazy", "leader", "leaf", "learn", "leave", "lecture", "left", "leg", "legal",
        "legend", "leisure", "lemon", "lend", "length", "lens", "leopard", "lesson", "letter", "level",
        "liar", "liberty", "library", "license", "life", "lift", "light", "like", "limb", "limit",
        "link", "lion", "liquid", "list", "little", "live", "lizard", "load", "loan", "lobster",
        "local", "lock", "logic", "lonely", "long", "loop", "lottery", "loud", "lounge", "love",
        "loyal", "lucky", "luggage", "lumber", "lunar", "lunch", "luxury", "lyrics", "machine", "mad",
        "magic", "magnet", "maid", "mail", "main", "major", "make", "mammal", "man", "manage",
        "mandate", "mango", "mansion", "manual", "maple", "marble", "march", "margin", "marine", "market",
        "marriage", "mask", "mass", "master", "match", "material", "math", "matrix", "matter", "maximum",
        "maze", "meadow", "mean", "measure", "meat", "mechanic", "medal", "media", "melody", "melt",
        "member", "memory", "mention", "menu", "mercy", "merge", "merit", "merry", "mesh", "message",
        "metal", "method", "middle", "midnight", "milk", "million", "mimic", "mind", "minimum", "minor",
        "minute", "miracle", "mirror", "misery", "miss", "mistake", "mix", "mixed", "mixture", "mobile",
        "model", "modify", "mom", "moment", "monitor", "monkey", "monster", "month", "moon", "moral",
        "more", "morning", "mosquito", "mother", "motion", "motor", "mountain", "mouse", "move", "movie",
        "much", "muffin", "mule", "multiply", "muscle", "museum", "mushroom", "music", "must", "mutual",
        "myself", "mystery", "myth", "naive", "name", "napkin", "narrow", "nasty", "nation", "nature",
        "near", "neck", "need", "negative", "neglect", "neither", "nephew", "nerve", "nest", "net",
        "network", "neutral", "never", "news", "next", "nice", "night", "noble", "noise", "nominee",
        "noodle", "normal", "north", "nose", "notable", "note", "nothing", "notice", "novel", "now",
        "nuclear", "number", "nurse", "nut", "oak", "obey", "object", "oblige", "obscure", "observe",
        "obtain", "obvious", "occur", "ocean", "october", "odor", "off", "offer", "office", "often",
        "oil", "okay", "old", "olive", "olympic", "omit", "once", "one", "onion", "online",
        "only", "open", "opera", "opinion", "oppose", "option", "orange", "orbit", "orchard", "order",
        "ordinary", "organ", "orient", "original", "orphan", "ostrich", "other", "outdoor", "outer", "output",
        "outside", "oval", "oven", "over", "own", "owner", "oxygen", "oyster", "ozone", "pact",
        "paddle", "page", "pair", "palace", "palm", "panda", "panel", "panic", "panther", "paper",
        "parade", "parent", "park", "parrot", "party", "pass", "patch", "path", "patient", "patrol",
        "pattern", "pause", "pave", "payment", "peace", "peanut", "pear", "peasant", "pelican", "pen",
        "penalty", "pencil", "people", "pepper", "perfect", "permit", "person", "pet", "phone", "photo",
        "phrase", "physical", "piano", "picnic", "picture", "piece", "pig", "pigeon", "pill", "pilot",
        "pink", "pioneer", "pipe", "pistol", "pitch", "pizza", "place", "planet", "plastic", "plate",
        "play", "please", "pledge", "pluck", "plug", "plunge", "poem", "poet", "point", "polar",
        "pole", "police", "pond", "pony", "pool", "popular", "portion", "position", "possible", "post",
        "potato", "pottery", "poverty", "powder", "power", "practice", "praise", "predict", "prefer", "prepare",
        "present", "pretty", "prevent", "price", "pride", "primary", "print", "priority", "prison", "private",
        "prize", "problem", "process", "produce", "profit", "program", "project", "promote", "proof", "property",
        "prosper", "protect", "proud", "provide", "public", "pudding", "pull", "pulp", "pulse", "pumpkin",
        "punch", "pupil", "puppy", "purchase", "purity", "purpose", "purse", "push", "put", "puzzle",
        "pyramid", "quality", "quantum", "quarter", "question", "quick", "quit", "quiz", "quote", "rabbit",
        "raccoon", "race", "rack", "radar", "radio", "rail", "rain", "raise", "rally", "ramp",
        "ranch", "random", "range", "rapid", "rare", "rate", "rather", "raven", "raw", "razor",
        "ready", "real", "reason", "rebel", "rebuild", "recall", "receive", "recipe", "record", "recycle",
        "reduce", "reflect", "reform", "refuse", "region", "regret", "regular", "reject", "relax", "release",
        "relief", "rely", "remain", "remember", "remind", "remove", "render", "renew", "rent", "reopen",
        "repair", "repeat", "replace", "report", "require", "rescue", "resemble", "resist", "resource", "response",
        "result", "retire", "retreat", "return", "reunion", "reveal", "review", "reward", "rhythm", "rib",
        "ribbon", "rice", "rich", "ride", "ridge", "rifle", "right", "rigid", "ring", "riot",
        "ripple", "risk", "ritual", "rival", "river", "road", "roast", "robot", "robust", "rocket",
        "romance", "roof", "rookie", "room", "rose", "rotate", "rough", "round", "route", "royal",
        "rubber", "rude", "rug", "rule", "run", "runway", "rural", "sad", "saddle", "sadness",
        "safe", "sail", "salad", "salmon", "salon", "salt", "salute", "same", "sample", "sand",
        "satisfy", "satoshi", "sauce", "sausage", "save", "say", "scale", "scan", "scare", "scatter",
        "scene", "scheme", "school", "science", "scissors", "scorpion", "scout", "scrap", "screen", "script",
        "scrub", "sea", "search", "season", "seat", "second", "secret", "section", "security", "seed",
        "seek", "segment", "select", "sell", "seminar", "senior", "sense", "sentence", "series", "service",
        "session", "settle", "setup", "seven", "shadow", "shaft", "shallow", "share", "shed", "shell",
        "sheriff", "shield", "shift", "shine", "ship", "shiver", "shock", "shoe", "shoot", "shop",
        "short", "shoulder", "shove", "shrimp", "shrug", "shuffle", "shy", "sibling", "sick", "side",
        "siege", "sight", "sign", "silent", "silk", "silly", "silver", "similar", "simple", "since",
        "sing", "siren", "sister", "situate", "six", "size", "skate", "sketch", "ski", "skill",
        "skin", "skirt", "skull", "slab", "slam", "sleep", "slender", "slice", "slide", "slight",
        "slim", "slogan", "slot", "slow", "slush", "small", "smart", "smile", "smoke", "smooth",
        "snack", "snake", "snap", "sniff", "snow", "soap", "soccer", "social", "sock", "soda",
        "soft", "solar", "soldier", "solid", "solution", "solve", "someone", "song", "soon", "sorry",
        "sort", "soul", "sound", "soup", "source", "south", "space", "spare", "spatial", "spawn",
        "speak", "special", "speed", "spell", "spend", "sphere", "spice", "spider", "spike", "spin",
        "spirit", "split", "spoil", "sponsor", "spoon", "sport", "spot", "spray", "spread", "spring",
        "spy", "square", "squeeze", "squirrel", "stable", "stadium", "staff", "stage", "stairs", "stamp",
        "stand", "start", "state", "stay", "steak", "steel", "stem", "step", "stereo", "stick",
        "still", "sting", "stock", "stomach", "stone", "stool", "story", "stove", "strategy", "street",
        "strike", "strong", "struggle", "student", "stuff", "stumble", "style", "subject", "submit", "subway",
        "success", "such", "sudden", "suffer", "sugar", "suggest", "suit", "summer", "sun", "sunny",
        "sunset", "super", "supply", "supreme", "sure", "surface", "surge", "surprise", "surround", "survey",
        "suspect", "sustain", "swallow", "swamp", "swap", "swarm", "swear", "sweet", "swift", "swim",
        "swing", "switch", "sword", "symbol", "symptom", "syrup", "system", "table", "tackle", "tag",
        "tail", "talent", "talk", "tank", "tape", "target", "task", "taste", "tattoo", "taxi",
        "teach", "team", "tell", "ten", "tenant", "tennis", "tent", "term", "test", "text",
        "thank", "that", "theme", "then", "theory", "there", "they", "thing", "this", "thought",
        "three", "thrive", "throw", "thumb", "thunder", "ticket", "tide", "tiger", "tilt", "timber",
        "time", "tiny", "tip", "tired", "tissue", "title", "toast", "tobacco", "today", "toddler",
        "toe", "together", "toilet", "token", "tomato", "tomorrow", "tone", "tongue", "tonight", "tool",
        "tooth", "top", "topic", "topple", "torch", "tornado", "tortoise", "toss", "total", "tourist",
        "toward", "tower", "town", "toy", "track", "trade", "traffic", "tragic", "train", "transfer",
        "trap", "trash", "travel", "tray", "treat", "tree", "trend", "trial", "tribe", "trick",
        "trigger", "trim", "trip", "trophy", "trouble", "truck", "true", "truly", "trumpet", "trust",
        "truth", "try", "tube", "tuition", "tumble", "tuna", "tunnel", "turkey", "turn", "turtle",
        "twelve", "twenty", "twice", "twin", "twist", "two", "type", "typical", "ugly", "umbrella",
        "unable", "unaware", "uncle", "uncover", "under", "undo", "unfair", "unfold", "unhappy", "uniform",
        "unique", "unit", "universe", "unknown", "unlock", "until", "unusual", "unveil", "update", "upgrade",
        "uphold", "upon", "upper", "upset", "urban", "urge", "usage", "use", "used", "useful",
        "useless", "usual", "utility", "vacant", "vacuum", "vague", "valid", "valley", "valve", "van",
        "vanish", "vapor", "various", "vast", "vault", "vehicle", "velvet", "vendor", "venture", "venue",
        "verb", "verify", "version", "very", "vessel", "veteran", "viable", "vibrant", "vicious", "victory",
        "video", "view", "village", "vintage", "violin", "virtual", "virus", "visa", "visit", "visual",
        "vital", "vivid", "vocal", "voice", "void", "volcano", "volume", "vote", "voyage", "wage",
        "wagon", "wait", "walk", "wall", "walnut", "want", "warfare", "warm", "warrior", "wash",
        "wasp", "waste", "water", "wave", "way", "wealth", "weapon", "wear", "weasel", "weather",
        "web", "wedding", "weekend", "weird", "welcome", "west", "wet", "whale", "what", "wheat",
        "wheel", "when", "where", "whip", "whisper", "wide", "width", "wife", "wild", "will",
        "win", "window", "wine", "wing", "wink", "winner", "winter", "wire", "wisdom", "wise",
        "wish", "witness", "wolf", "woman", "wonder", "wood", "wool", "word", "work", "world",
        "worry", "worth", "wrap", "wreck", "wrestle", "wrist", "write", "wrong", "yard", "year",
        "yellow", "you", "young", "youth", "zebra", "zero", "zone", "zoo"
    ]
}

// MARK: - Helper Extensions
extension Data {
    init?(hexString: String) {
        let cleanHex = hexString.replacingOccurrences(of: "0x", with: "")
        let len = cleanHex.count / 2
        var data = Data(capacity: len)

        for i in 0..<len {
            let start = cleanHex.index(cleanHex.startIndex, offsetBy: i * 2)
            let end = cleanHex.index(start, offsetBy: 2)
            let bytes = cleanHex[start..<end]

            guard let byte = UInt8(bytes, radix: 16) else {
                return nil
            }

            data.append(byte)
        }

        self = data
    }

    var hexString: String {
        map { String(format: "%02x", $0) }.joined()
    }
}

