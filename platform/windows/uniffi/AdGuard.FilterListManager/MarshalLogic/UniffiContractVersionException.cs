namespace AdGuard.FilterListManager.MarshalLogic
{

    /// <summary>
    /// Occurs when the rust-backed functions have wrong version.
    /// </summary>
    /// <seealso cref="UniffiException" />
    public class UniffiContractVersionException : UniffiException
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="UniffiContractVersionException"/> class.
        /// </summary>
        /// <param name="message">The message.</param>
        public UniffiContractVersionException(string message)
            : base(message) { }
    }
}