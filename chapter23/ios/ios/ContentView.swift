//
//  ContentView.swift
//  ios
//
//  Created by ranger on 2024/2/25.
//

import SwiftUI
import SwiftData

struct ContentView: View {
    @Environment(\.modelContext) private var modelContext
    @Query private var items: [Item]


    var body: some View {
        let r = RsProject()
        Text("\(r.hello())")
                    .padding()
    }
}
