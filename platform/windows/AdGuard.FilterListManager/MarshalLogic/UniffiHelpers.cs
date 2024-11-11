namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class UniffiHelpers
    {
        public delegate void RustCallAction(ref RustCallStatus status);
        public delegate TU RustCallFunc<out TU>(ref RustCallStatus status);

        // Call a rust function that returns a Result<>.  Pass in the Error class companion that corresponds to the Err
        public static TU RustCallWithError<TU, TE>(
            ICallStatusErrorHandler<TE> errorHandler,
            RustCallFunc<TU> callback
        )
            where TE : UniffiException
        {
            var status = new RustCallStatus();
            var returnValue = callback(ref status);
            if (status.IsSuccess())
            {
                return returnValue;
            }

            if (status.IsError())
            {
                throw errorHandler.Lift(status.error_buf);
            }

            if (status.IsPanic())
            {
                // when the rust code sees a panic, it tries to construct a rust-buffer
                // with the message.  but if that code panics, then it just sends back
                // an empty buffer.
                if (status.error_buf.len > 0)
                {
                    throw new PanicException(FfiConverterString.Instance.Lift(status.error_buf));
                }

                throw new PanicException("Rust panic");
            }

            throw new InternalException($"Unknown rust call status: {status.code}");
        }

        // Call a rust function that returns a Result<>.  Pass in the Error class companion that corresponds to the Err
        public static void RustCallWithError<TE>(
            ICallStatusErrorHandler<TE> errorHandler,
            RustCallAction callback
        )
            where TE : UniffiException
        {
            RustCallWithError(
                errorHandler,
                (ref RustCallStatus status) =>
                {
                    callback(ref status);
                    return 0;
                }
            );
        }

        // Call a rust function that returns a plain value
        public static TU RustCall<TU>(RustCallFunc<TU> callback)
        {
            return RustCallWithError(NullCallStatusErrorHandler.Instance, callback);
        }

        // Call a rust function that returns a plain value
        public static void RustCall(RustCallAction callback)
        {
            RustCall(
                (ref RustCallStatus status) =>
                {
                    callback(ref status);
                    return 0;
                }
            );
        }
    }
}