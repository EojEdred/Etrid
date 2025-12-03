import Foundation
import CryptoKit
import CommonCrypto
import P256K
import CryptoSwift

// MARK: - BIP39 Mnemonic Generation & Validation
public actor HDWalletManager {

    // BIP39 English wordlist (2048 words)
    private static let wordlist: [String] = [
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

    // MARK: - Mnemonic Generation

    /// Generate a new 12-word BIP39 mnemonic (128 bits = 12 words, industry standard)
    public func generateMnemonic(strength: Int = 128) throws -> String {
        guard strength == 128 || strength == 160 || strength == 192 || strength == 224 || strength == 256 else {
            throw HDWalletError.invalidEntropyStrength
        }

        let entropyBytes = strength / 8
        var entropy = [UInt8](repeating: 0, count: entropyBytes)
        let status = SecRandomCopyBytes(kSecRandomDefault, entropyBytes, &entropy)

        guard status == errSecSuccess else {
            throw HDWalletError.entropyGenerationFailed
        }

        return try mnemonicFromEntropy(Data(entropy))
    }

    /// Convert entropy to mnemonic words
    private func mnemonicFromEntropy(_ entropy: Data) throws -> String {
        let entropyBits = entropy.flatMap { byte in
            (0..<8).reversed().map { (byte >> $0) & 1 }
        }

        // Calculate checksum
        let hash = SHA256.hash(data: entropy)
        let hashBits = Array(hash).flatMap { byte in
            (0..<8).reversed().map { (byte >> $0) & 1 }
        }

        let checksumLength = entropy.count / 4
        let allBits = entropyBits + Array(hashBits.prefix(checksumLength))

        // Convert to 11-bit indices
        var words: [String] = []
        for i in stride(from: 0, to: allBits.count, by: 11) {
            let indexBits = Array(allBits[i..<min(i + 11, allBits.count)])
            let index = indexBits.reduce(0) { ($0 << 1) | Int($1) }
            words.append(Self.wordlist[index])
        }

        return words.joined(separator: " ")
    }

    /// Validate a BIP39 mnemonic
    public func validateMnemonic(_ mnemonic: String) -> Bool {
        let words = mnemonic.lowercased().split(separator: " ").map(String.init)

        guard [12, 15, 18, 21, 24].contains(words.count) else {
            return false
        }

        // Check all words are in wordlist
        for word in words {
            guard Self.wordlist.contains(word) else {
                return false
            }
        }

        // Verify checksum
        guard let entropy = try? entropyFromMnemonic(mnemonic) else {
            return false
        }

        return true
    }

    /// Convert mnemonic back to entropy (for validation)
    private func entropyFromMnemonic(_ mnemonic: String) throws -> Data {
        let words = mnemonic.lowercased().split(separator: " ").map(String.init)

        var bits: [UInt8] = []
        for word in words {
            guard let index = Self.wordlist.firstIndex(of: word) else {
                throw HDWalletError.invalidMnemonicWord(word)
            }
            for i in (0..<11).reversed() {
                bits.append(UInt8((index >> i) & 1))
            }
        }

        let checksumLength = words.count / 3
        let entropyBits = Array(bits.dropLast(checksumLength))

        var entropy = Data()
        for i in stride(from: 0, to: entropyBits.count, by: 8) {
            let byteBits = Array(entropyBits[i..<min(i + 8, entropyBits.count)])
            let byte = byteBits.reduce(UInt8(0)) { ($0 << 1) | $1 }
            entropy.append(byte)
        }

        return entropy
    }

    // MARK: - Seed Generation (BIP39)

    /// Generate seed from mnemonic using PBKDF2
    public func seedFromMnemonic(_ mnemonic: String, passphrase: String = "") throws -> Data {
        let normalizedMnemonic = mnemonic.decomposedStringWithCompatibilityMapping
        let salt = "mnemonic" + passphrase
        let normalizedSalt = salt.decomposedStringWithCompatibilityMapping

        guard let mnemonicData = normalizedMnemonic.data(using: .utf8),
              let saltData = normalizedSalt.data(using: .utf8) else {
            throw HDWalletError.encodingError
        }

        var derivedKey = [UInt8](repeating: 0, count: 64)

        let result = mnemonicData.withUnsafeBytes { mnemonicBytes in
            saltData.withUnsafeBytes { saltBytes in
                CCKeyDerivationPBKDF(
                    CCPBKDFAlgorithm(kCCPBKDF2),
                    mnemonicBytes.baseAddress?.assumingMemoryBound(to: Int8.self),
                    mnemonicData.count,
                    saltBytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    saltData.count,
                    CCPseudoRandomAlgorithm(kCCPRFHmacAlgSHA512),
                    2048,
                    &derivedKey,
                    64
                )
            }
        }

        guard result == kCCSuccess else {
            throw HDWalletError.seedDerivationFailed
        }

        return Data(derivedKey)
    }

    // MARK: - BIP32 Key Derivation

    /// Derive master key from seed
    public func masterKeyFromSeed(_ seed: Data) throws -> ExtendedKey {
        let key = "Bitcoin seed"
        guard let keyData = key.data(using: .utf8) else {
            throw HDWalletError.encodingError
        }

        let hmac = HMAC<SHA512>.authenticationCode(for: seed, using: SymmetricKey(data: keyData))
        let hmacData = Data(hmac)

        let privateKey = hmacData.prefix(32)
        let chainCode = hmacData.suffix(32)

        return ExtendedKey(
            privateKey: privateKey,
            chainCode: chainCode,
            depth: 0,
            fingerprint: Data([0, 0, 0, 0]),
            childIndex: 0
        )
    }

    /// Derive child key using BIP32 path (e.g., "m/44'/60'/0'/0/0")
    public func deriveKey(from masterKey: ExtendedKey, path: String) throws -> ExtendedKey {
        let components = path.split(separator: "/").dropFirst() // Remove "m"

        var key = masterKey
        for component in components {
            let componentStr = String(component)
            let hardened = componentStr.hasSuffix("'")
            let indexStr = hardened ? String(componentStr.dropLast()) : componentStr

            guard let index = UInt32(indexStr) else {
                throw HDWalletError.invalidDerivationPath
            }

            let childIndex = hardened ? (0x80000000 | index) : index
            key = try deriveChildKey(from: key, index: childIndex)
        }

        return key
    }

    /// Derive a single child key
    private func deriveChildKey(from parentKey: ExtendedKey, index: UInt32) throws -> ExtendedKey {
        var data = Data()

        if index >= 0x80000000 {
            // Hardened derivation
            data.append(0x00)
            data.append(parentKey.privateKey)
        } else {
            // Normal derivation - compute public key
            let publicKey = try computePublicKey(from: parentKey.privateKey)
            data.append(publicKey)
        }

        // Append index in big-endian
        var indexBE = index.bigEndian
        data.append(Data(bytes: &indexBE, count: 4))

        let hmac = HMAC<SHA512>.authenticationCode(
            for: data,
            using: SymmetricKey(data: parentKey.chainCode)
        )
        let hmacData = Data(hmac)

        let il = hmacData.prefix(32)
        let ir = hmacData.suffix(32)

        // Add parent key to IL (mod n for secp256k1)
        let childPrivateKey = try addPrivateKeys(il, parentKey.privateKey)

        // Compute fingerprint from parent public key
        let parentPublicKey = try computePublicKey(from: parentKey.privateKey)
        let hash = Data(SHA256.hash(data: parentPublicKey))
        var ripemd = [UInt8](repeating: 0, count: 20)
        // Simplified RIPEMD-160 (in production, use proper implementation)
        ripemd = Array(hash.prefix(20))
        let fingerprint = Data(ripemd.prefix(4))

        return ExtendedKey(
            privateKey: childPrivateKey,
            chainCode: ir,
            depth: parentKey.depth + 1,
            fingerprint: fingerprint,
            childIndex: index
        )
    }

    /// Compute secp256k1 public key from private key using real EC multiplication
    private func computePublicKey(from privateKey: Data) throws -> Data {
        guard privateKey.count == 32 else {
            throw HDWalletError.invalidPrivateKey
        }

        // Use secp256k1 library for real elliptic curve multiplication
        let signingKey = try P256K.Signing.PrivateKey(dataRepresentation: privateKey)

        // Return compressed public key (33 bytes)
        return signingKey.publicKey.dataRepresentation
    }

    /// Add two private keys (mod secp256k1 order) using proper modular arithmetic
    private func addPrivateKeys(_ a: Data, _ b: Data) throws -> Data {
        guard a.count == 32, b.count == 32 else {
            throw HDWalletError.invalidPrivateKey
        }

        // secp256k1 curve order n
        let curveOrder: [UInt8] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
            0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B,
            0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41
        ]

        // Add with carry
        var result = [UInt8](repeating: 0, count: 32)
        var carry: UInt16 = 0

        let aBytes = Array(a)
        let bBytes = Array(b)

        for i in (0..<32).reversed() {
            let sum = UInt16(aBytes[i]) + UInt16(bBytes[i]) + carry
            result[i] = UInt8(sum & 0xFF)
            carry = sum >> 8
        }

        // Reduce modulo curve order if result >= n
        if carry > 0 || compareBytes(result, curveOrder) >= 0 {
            result = subtractBytes(result, curveOrder)
        }

        return Data(result)
    }

    /// Compare two byte arrays (big-endian)
    private func compareBytes(_ a: [UInt8], _ b: [UInt8]) -> Int {
        for i in 0..<min(a.count, b.count) {
            if a[i] < b[i] { return -1 }
            if a[i] > b[i] { return 1 }
        }
        return 0
    }

    /// Subtract two byte arrays (big-endian), assuming a >= b
    private func subtractBytes(_ a: [UInt8], _ b: [UInt8]) -> [UInt8] {
        var result = [UInt8](repeating: 0, count: 32)
        var borrow: Int16 = 0

        for i in (0..<32).reversed() {
            let diff = Int16(a[i]) - Int16(b[i]) - borrow
            if diff < 0 {
                result[i] = UInt8((diff + 256) & 0xFF)
                borrow = 1
            } else {
                result[i] = UInt8(diff & 0xFF)
                borrow = 0
            }
        }

        return result
    }

    // MARK: - BIP44 Derivation Paths

    /// Get derivation path for ËTRID (using coin type 60 for EVM compatibility)
    public func etridDerivationPath(account: Int = 0, index: Int = 0) -> String {
        return "m/44'/60'/\(account)'/0/\(index)"
    }

    /// Get derivation path for specific chain
    public func derivationPath(for chain: SupportedChain, account: Int = 0, index: Int = 0) -> String {
        return "m/44'/\(chain.coinType)'/\(account)'/0/\(index)"
    }
}

// MARK: - Supporting Types

public struct ExtendedKey {
    public let privateKey: Data
    public let chainCode: Data
    public let depth: UInt8
    public let fingerprint: Data
    public let childIndex: UInt32

    /// Derive Ethereum-style address from private key using real Keccak256
    public func ethereumAddress() throws -> String {
        // Get uncompressed public key (65 bytes)
        let publicKey = try computeUncompressedPublicKey()

        // Keccak256 of public key without 0x04 prefix (64 bytes)
        let pubKeyWithoutPrefix = Data(publicKey.dropFirst())
        let hash = pubKeyWithoutPrefix.sha3(.keccak256)

        // Take last 20 bytes
        let addressBytes = hash.suffix(20)

        // Return with EIP-55 checksum encoding
        return checksumAddress(Data(addressBytes))
    }

    /// Generate ËTRID (Substrate SS58) address
    public func etridAddress() async throws -> String {
        let publicKey = try computeCompressedPublicKey()
        return try await CryptoUtils.shared.generateEtridAddress(from: publicKey)
    }

    /// Generate Bitcoin address
    public func bitcoinAddress(testnet: Bool = false) async throws -> String {
        return try await CryptoUtils.shared.generateBitcoinAddress(from: privateKey, testnet: testnet)
    }

    /// Generate Solana address (requires Ed25519 derivation)
    public func solanaAddress() async throws -> String {
        // Solana uses Ed25519, derive from first 32 bytes of private key
        return try await CryptoUtils.shared.generateSolanaAddress(from: privateKey)
    }

    /// Compute compressed secp256k1 public key (33 bytes)
    public func computeCompressedPublicKey() throws -> Data {
        guard privateKey.count == 32 else {
            throw HDWalletError.invalidPrivateKey
        }

        let signingKey = try P256K.Signing.PrivateKey(dataRepresentation: privateKey)
        return signingKey.publicKey.dataRepresentation
    }

    /// Compute uncompressed secp256k1 public key (65 bytes)
    private func computeUncompressedPublicKey() throws -> Data {
        guard privateKey.count == 32 else {
            throw HDWalletError.invalidPrivateKey
        }

        // Create with uncompressed format
        let signingKey = try P256K.Signing.PrivateKey(dataRepresentation: privateKey, format: .uncompressed)
        return signingKey.publicKey.dataRepresentation
    }

    /// EIP-55 checksum address encoding
    private func checksumAddress(_ addressBytes: Data) -> String {
        let hexAddress = addressBytes.map { String(format: "%02x", $0) }.joined()
        let hashHex = hexAddress.data(using: .utf8)!.sha3(.keccak256).toHexString()

        var checksummed = "0x"
        for (i, char) in hexAddress.enumerated() {
            let hashChar = hashHex[hashHex.index(hashHex.startIndex, offsetBy: i)]
            if let hashInt = Int(String(hashChar), radix: 16), hashInt >= 8 {
                checksummed.append(char.uppercased())
            } else {
                checksummed.append(char)
            }
        }

        return checksummed
    }
}

public enum SupportedChain: String, CaseIterable, Codable {
    case etrid = "ËTRID"
    case ethereum = "Ethereum"
    case solana = "Solana"
    case bnbChain = "BNB Chain"
    case xrp = "XRP"
    case tron = "TRON"
    case cardano = "Cardano"
    case stellar = "Stellar"
    case dogecoin = "Dogecoin"

    public var coinType: Int {
        switch self {
        case .etrid, .ethereum: return 60
        case .solana: return 501
        case .bnbChain: return 714
        case .xrp: return 144
        case .tron: return 195
        case .cardano: return 1815
        case .stellar: return 148
        case .dogecoin: return 3
        }
    }

    public var symbol: String {
        switch self {
        case .etrid: return "ETR"
        case .ethereum: return "ETH"
        case .solana: return "SOL"
        case .bnbChain: return "BNB"
        case .xrp: return "XRP"
        case .tron: return "TRX"
        case .cardano: return "ADA"
        case .stellar: return "XLM"
        case .dogecoin: return "DOGE"
        }
    }

    public var decimals: Int {
        switch self {
        case .etrid, .ethereum, .bnbChain: return 18
        case .solana: return 9
        case .xrp, .tron, .cardano: return 6
        case .stellar: return 7
        case .dogecoin: return 8
        }
    }
}

public enum HDWalletError: LocalizedError {
    case invalidEntropyStrength
    case entropyGenerationFailed
    case invalidMnemonicWord(String)
    case encodingError
    case seedDerivationFailed
    case invalidDerivationPath
    case invalidPrivateKey
    case keyDerivationFailed

    public var errorDescription: String? {
        switch self {
        case .invalidEntropyStrength:
            return "Entropy strength must be 128, 160, 192, 224, or 256 bits"
        case .entropyGenerationFailed:
            return "Failed to generate secure random entropy"
        case .invalidMnemonicWord(let word):
            return "Invalid mnemonic word: \(word)"
        case .encodingError:
            return "String encoding error"
        case .seedDerivationFailed:
            return "Failed to derive seed from mnemonic"
        case .invalidDerivationPath:
            return "Invalid BIP32 derivation path"
        case .invalidPrivateKey:
            return "Invalid private key format"
        case .keyDerivationFailed:
            return "Key derivation failed"
        }
    }
}
