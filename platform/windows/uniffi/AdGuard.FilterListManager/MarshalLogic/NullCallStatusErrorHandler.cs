namespace AdGuard.FilterListManager.MarshalLogic
{
    class NullCallStatusErrorHandler : ICallStatusErrorHandler<UniffiException>
    {
        public static NullCallStatusErrorHandler Instance = new NullCallStatusErrorHandler();

        /// <summary>
        /// Lifts the specified error buf.
        /// </summary>
        /// <param name="errorBuf">The error buf.</param>
        /// <returns></returns>
        public UniffiException Lift(RustBuffer errorBuf)
        {
            RustBuffer.Free(errorBuf);
            return new UndeclaredErrorException(
                "library has returned an error not declared in UNIFFI interface file"
            );
        }
    }
}