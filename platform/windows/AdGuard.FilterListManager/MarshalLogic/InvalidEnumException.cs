namespace AdGuard.FilterListManager.MarshalLogic
{
    public class InvalidEnumException : InternalException
    {
        public InvalidEnumException(string message)
            : base(message) { }
    }
}