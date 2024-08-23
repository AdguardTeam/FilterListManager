namespace AdGuard.FilterListManager.MarshalLogic
{
    public class PanicException : UniffiException
    {
        public PanicException(string message)
            : base(message) { }
    }
}