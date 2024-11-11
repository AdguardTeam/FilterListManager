using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Base FFI-related exception.
    /// </summary>
    /// <seealso cref="Exception" />
    public class UniffiException : Exception
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="UniffiException"/> class.
        /// </summary>
        public UniffiException()
            : base() { }

        /// <summary>
        /// Initializes a new instance of the <see cref="UniffiException"/> class.
        /// </summary>
        /// <param name="message">The message that describes the error.</param>
        public UniffiException(string message)
            : base(message) { }
    }
}