//
//  AGOuterErrorVariant.swift
//  FLMLocal
//

public enum AGOuterErrorVariant: Error {
    case CannotOpenDatabase
    case NotADatabase
    case DiskFull
    case EntityNotFound(Int64)
    case PathNotFound(String)
    case PathHasDeniedPermission(String)
    case PathAlreadyExists(String)
    case TimedOut
    case HttpClientNetworkError
    case HttpStrict200Response(UInt32, String)
    case HttpClientBodyRecoveryFailed
    case FilterContentIsLikelyNotAFilter
    case FilterParserError
    case FieldIsEmpty(String)
    case DatabaseBusy
    case Mutex
    case Other
}

extension AGOuterErrorVariant {
    init(from: inout FilterListManager_AGOuterError) {
        guard let thisCase = from.error else {
            from.message = "Error variant is nil"

            self = Self.Other

            return
        }

        switch thisCase {
        case .cannotOpenDatabase:
            self = Self.CannotOpenDatabase
        case .notADatabase:
            self = Self.NotADatabase
        case .diskFull:
            self = Self.DiskFull
        case .entityNotFound(let container):
            self = Self.EntityNotFound(container.entityID)
        case .pathNotFound(let container):
            self = Self.PathNotFound(container.path)
        case .pathHasDeniedPermission(let container):
            self = Self.PathHasDeniedPermission(container.path)
        case .pathAlreadyExists(let container):
            self = Self.PathAlreadyExists(container.path)
        case .timedOut(_):
            self = Self.TimedOut
        case .httpClientNetworkError(_):
            self = Self.HttpClientNetworkError
        case .httpStrict200Response(let container):
            self = Self.HttpStrict200Response(container.statusCode, container.url)
        case .httpClientBodyRecoveryFailed(_):
            self = Self.HttpClientBodyRecoveryFailed
        case .filterContentIsLikelyNotAFilter(_):
            self = Self.FilterContentIsLikelyNotAFilter
        case .filterParserError(_):
            self = Self.FilterParserError
        case .fieldIsEmpty(let container):
            self = Self.FieldIsEmpty(container.fieldName)
        case .mutex(_):
            self = Self.Mutex
        case .other(_):
            self = Self.Other
        case .databaseBusy(_):
            self = Self.DatabaseBusy
        }
    }
}
