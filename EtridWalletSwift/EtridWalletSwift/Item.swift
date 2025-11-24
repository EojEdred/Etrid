//
//  Item.swift
//  EtridWalletSwift
//
//  Created by macbook on 11/22/25.
//

import Foundation
import SwiftData

@Model
final class Item {
    var timestamp: Date
    
    init(timestamp: Date) {
        self.timestamp = timestamp
    }
}
