using System;

namespace AdGuard.FilterListManagerProtobuf.Api.Exceptions
{
    /// <summary>
    /// This exception is thrown for any reason within the backend client 
    /// </summary>
    public class FilterListManagerNotInitializedException : FilterListManagerBaseException
    {
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerNotInitializedException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        public FilterListManagerNotInitializedException(string errorMessage) 
            : base(errorMessage) {}
        
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerNotInitializedException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        /// <param name="innerException">Inner exception</param>
        public FilterListManagerNotInitializedException(string errorMessage, Exception innerException) 
            : base(errorMessage, innerException) {}
    }
}