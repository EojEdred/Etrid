//
//  NFT.swift
//  EtridWallet
//
//  NFT data models
//

import Foundation

struct NFT: Identifiable, Codable {
    let id: String
    let name: String
    let collection: String
    let imageURL: String
    let price: String
    let chain: String
    var description: String?
    var attributes: [NFTAttribute]?
    var owner: String?
    var creator: String?

    var displayPrice: String {
        price
    }
}

struct NFTAttribute: Identifiable, Codable {
    let id = UUID()
    let traitType: String
    let value: String
    let rarity: Double?

    enum CodingKeys: String, CodingKey {
        case traitType = "trait_type"
        case value
        case rarity
    }
}

struct NFTCollection: Identifiable, Codable {
    let id: String
    let name: String
    let description: String
    let imageURL: String
    let floorPrice: String
    let totalVolume: String
    let itemCount: Int
}

// MARK: - Mock Data
extension NFT {
    static let mockNFTs: [NFT] = [
        NFT(
            id: "1",
            name: "Cosmic Tiger #42",
            collection: "Cosmic Cats",
            imageURL: "https://picsum.photos/400/400?random=1",
            price: "2.5 ÉTR",
            chain: "Primearc Core Chain",
            description: "A rare cosmic tiger from the Cosmic Cats collection",
            attributes: [
                NFTAttribute(traitType: "Background", value: "Cosmic", rarity: 0.15),
                NFTAttribute(traitType: "Eyes", value: "Laser", rarity: 0.05),
                NFTAttribute(traitType: "Fur", value: "Galaxy", rarity: 0.08)
            ]
        ),
        NFT(
            id: "2",
            name: "Cyber Punk #128",
            collection: "Future Punks",
            imageURL: "https://picsum.photos/400/400?random=2",
            price: "5.0 ÉTR",
            chain: "Primearc Core Chain",
            description: "Punk from the digital future",
            attributes: [
                NFTAttribute(traitType: "Type", value: "Cyber", rarity: 0.12),
                NFTAttribute(traitType: "Accessory", value: "Visor", rarity: 0.18)
            ]
        ),
        NFT(
            id: "3",
            name: "Abstract Art #007",
            collection: "Modern Art",
            imageURL: "https://picsum.photos/400/400?random=3",
            price: "1.8 ÉTR",
            chain: "Primearc Core Chain",
            description: "Abstract digital masterpiece"
        ),
        NFT(
            id: "4",
            name: "Digital Landscape #55",
            collection: "Landscapes",
            imageURL: "https://picsum.photos/400/400?random=4",
            price: "3.2 ÉTR",
            chain: "Primearc Core Chain",
            description: "Breathtaking digital landscape"
        )
    ]
}
