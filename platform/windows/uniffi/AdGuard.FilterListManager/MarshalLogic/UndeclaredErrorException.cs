namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Un unknown <see cref="UniffiException"/>
    /// </summary>
    /// <seealso cref="AdGuard.FilterListManager.MarshalLogic.UniffiException" />
    public class UndeclaredErrorException : UniffiException
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="UndeclaredErrorException"/> class.
        /// </summary>
        /// <param name="message">The message.</param>
        public UndeclaredErrorException(string message)
            : base(message) { }
    }
}