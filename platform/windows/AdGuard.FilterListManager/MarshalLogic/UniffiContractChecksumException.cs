namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Occurs when the rust-backed functions have wrong checksum.
    /// </summary>
    /// <seealso cref="UniffiException" />
    public class UniffiContractChecksumException : UniffiException
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="UniffiContractChecksumException"/> class.
        /// </summary>
        /// <param name="message">The message.</param>
        public UniffiContractChecksumException(string message)
            : base(message) { }
    }
}