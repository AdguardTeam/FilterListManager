//
//  FLMFacadeError.swift
//  FLMLocal
//

public enum FLMFacadeError: Error {
    case objectIsNotInited
    case noDataOnResponse
    case rustResponseAsNullptr
}
