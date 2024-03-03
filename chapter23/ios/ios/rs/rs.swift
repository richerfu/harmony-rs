//
//  rs.swift
//  ios
//
//  Created by ranger on 2024/3/3.
//

import Foundation

class RsProject {

    func hello() -> String {
        let result = hello_rust()
        let swift_result = String(cString: result!)
        return swift_result
    }
}
