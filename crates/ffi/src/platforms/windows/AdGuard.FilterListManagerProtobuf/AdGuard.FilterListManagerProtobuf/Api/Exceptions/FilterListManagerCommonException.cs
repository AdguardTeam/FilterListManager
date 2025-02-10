using System;

namespace AdGuard.FilterListManagerProtobuf.Api.Exceptions
{
    /// <summary>
    /// This exception is thrown for any reason within the backend client 
    /// </summary>
    public class FilterListManagerCommonException : FilterListManagerBaseException
    {
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerCommonException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        public FilterListManagerCommonException(string errorMessage) 
            : base(errorMessage) {}
        
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerCommonException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        /// <param name="innerException">Inner exception</param>
        public FilterListManagerCommonException(string errorMessage, Exception innerException) 
            : base(errorMessage, innerException) {}
    }
}