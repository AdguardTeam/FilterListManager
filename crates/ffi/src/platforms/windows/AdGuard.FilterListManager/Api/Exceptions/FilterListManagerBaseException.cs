using System;

namespace AdGuard.FilterListManager.Api.Exceptions
{
    /// <summary>
    /// This exception is thrown for any reason within the backend client 
    /// </summary>
    public abstract class FilterListManagerBaseException : InvalidOperationException
    {
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerBaseException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        public FilterListManagerBaseException(string errorMessage) 
            : base(errorMessage) {}
        
        /// <summary>
        /// Creates an instance of <see cref="FilterListManagerBaseException"/> according to the passed parameters
        /// </summary>
        /// <param name="errorMessage">Error message</param>
        /// <param name="innerException">Inner exception</param>
        public FilterListManagerBaseException(string errorMessage, Exception innerException) 
            : base(errorMessage, innerException) {}
    }
}