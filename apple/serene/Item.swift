//
//  Item.swift
//  serene
//
//  Created by Zixuan Zhang on 7/27/24.
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
