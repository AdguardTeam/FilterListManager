//
//  AGOuterError.swift
//  FLMLocal
//

import Foundation
import AdGuardFLM

/// Error container.
public struct AGOuterError: Error, LocalizedError {
    /// String representation of error.
    public let message: String
    /// Error type with details.
    public let variant: AGOuterErrorVariant

    public var localizedDescription: String {
        self.message
    }

    public var errorDescription: String? {
        self.message
    }

    init(from: FilterListManager_AGOuterError) {
        var error = from
        self.variant = AGOuterErrorVariant(from: &error)
        self.message = error.message
    }
}
