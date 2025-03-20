namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Special internal exception.
    /// </summary>
    /// <seealso cref="UniffiException" />
    public class InternalException : UniffiException
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="InternalException"/> class.
        /// </summary>
        /// <param name="message">The message.</param>
        public InternalException(string message)
            : base(message) { }
    }
}