using System;

namespace AdGuard.FilterListManagerProtobuf.Api.Exceptions
{
    /// <summary>
    /// This exception is thrown if invalid discriminant was obtained
    /// </summary>
    public class FilterListManagerInvalidDiscriminantException : FilterListManagerBaseException
    {
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerInvalidDiscriminantException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        public FilterListManagerInvalidDiscriminantException(string errorMessage) 
            : base(errorMessage) {}
        
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerCommonException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        /// <param name="innerException">Inner exception</param>
        public FilterListManagerInvalidDiscriminantException(string errorMessage, Exception innerException) 
            : base(errorMessage, innerException) {}
    }
}