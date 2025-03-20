namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Memory allocation-related exception.
    /// </summary>
    /// <seealso cref="UniffiException" />
    public class AllocationException : UniffiException
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="AllocationException"/> class.
        /// </summary>
        /// <param name="message">The message.</param>
        public AllocationException(string message)
            : base(message) { }
    }
}