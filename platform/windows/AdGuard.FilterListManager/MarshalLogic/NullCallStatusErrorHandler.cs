namespace AdGuard.FilterListManager.MarshalLogic
{
    class NullCallStatusErrorHandler : CallStatusErrorHandler<UniffiException>
    {
        public static NullCallStatusErrorHandler INSTANCE = new NullCallStatusErrorHandler();

        public UniffiException Lift(RustBuffer error_buf)
        {
            RustBuffer.Free(error_buf);
            return new UndeclaredErrorException(
                "library has returned an error not declared in UNIFFI interface file"
            );
        }
    }
}