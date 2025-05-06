package com.adguard.flm.exceptions

import com.adguard.flm.protobuf.OuterError

data class FilterListManagerException(
    val outerError: OuterError.AGOuterError
) : Exception(outerError.message)
