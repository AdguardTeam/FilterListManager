using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class UniffiException : Exception
    {
        public UniffiException()
            : base() { }

        public UniffiException(string message)
            : base(message) { }
    }
}