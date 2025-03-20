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
        public UpdateFilterError(int filterId, string message)
        {
            this.filterId = filterId;
            this.message = message;
        }

        /// <summary>
        /// Gets or sets the filter identifier.
        /// </summary>
        public int filterId { get; set; }

        /// <summary>
        /// Gets or sets the message of the error.
        /// </summary>
        public string message { get; set; }
    }
}