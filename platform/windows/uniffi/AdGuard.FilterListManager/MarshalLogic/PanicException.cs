namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Panic-related <see cref="UniffiException"/>
    /// </summary>
    /// <seealso cref="UniffiException" />
    public class PanicException : UniffiException
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="PanicException"/> class.
        /// </summary>
        /// <param name="message">The message.</param>
        public PanicException(string message)
            : base(message) { }
    }
}