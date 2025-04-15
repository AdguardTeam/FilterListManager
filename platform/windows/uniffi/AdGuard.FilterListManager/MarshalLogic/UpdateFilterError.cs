namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Filter update-related entity
    /// </summary>
    public class UpdateFilterError
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="UpdateFilterError"/> class.
        /// </summary>
        /// <param name="filterId">The filter identifier.</param>
        /// <param name="message">The message of the error.</param>
        /// <param name="filterUrl">The filter URL.</param>
        /// <param name="httpClientError">The HTTP client error.</param>
        public UpdateFilterError(int filterId, string message, string filterUrl, string httpClientError)
        {
            FilterId = filterId;
            Message = message;
            FilterUrl = filterUrl;
            HttpClientError = httpClientError;
        }

        /// <summary>
        /// Gets the filter identifier.
        /// </summary>
        public int FilterId { get; }

        /// <summary>
        /// Gets the message of the error.
        /// </summary>
        public string Message { get; }

        /// <summary>
        /// Gets the filter URL.
        /// </summary>
        public string FilterUrl { get; }

        /// <summary>
        /// Gets the HTTP client error.
        /// </summary>
        public string HttpClientError { get; }
    }
}