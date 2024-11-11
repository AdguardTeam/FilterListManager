namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Enum-related internal exception.
    /// </summary>
    /// <seealso cref="InternalException" />
    public class InvalidEnumException : InternalException
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="InvalidEnumException"/> class.
        /// </summary>
        /// <param name="message">The message.</param>
        public InvalidEnumException(string message)
            : base(message) { }
    }
}