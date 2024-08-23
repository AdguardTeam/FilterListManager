namespace AdGuard.FilterListManager.MarshalLogic
{
    public class UndeclaredErrorException : UniffiException
    {
        public UndeclaredErrorException(string message)
            : base(message) { }
    }
}