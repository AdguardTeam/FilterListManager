namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeUpdateFilterError : FfiConverterRustBuffer<UpdateFilterError>
    {
        public static FfiConverterTypeUpdateFilterError Instance =
            new FfiConverterTypeUpdateFilterError();

        public override UpdateFilterError Read(BigEndianStream stream)
        {
            return new UpdateFilterError(
                filterId: FfiConverterInt32.Instance.Read(stream),
                message: FfiConverterString.Instance.Read(stream),
                filterUrl: FfiConverterOptionalString.Instance.Read(stream),
                httpClientError: FfiConverterOptionalString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(UpdateFilterError value)
        {
            return FfiConverterInt32.Instance.AllocationSize(value.filterId)
                   + FfiConverterString.Instance.AllocationSize(value.message)
                   + FfiConverterOptionalString.Instance.AllocationSize(value.filterUrl)
                   + FfiConverterOptionalString.Instance.AllocationSize(value.httpClientError);
        }

        public override void Write(UpdateFilterError value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.filterId, stream);
            FfiConverterString.Instance.Write(value.message, stream);
            FfiConverterOptionalString.Instance.Write(value.filterUrl, stream);
            FfiConverterOptionalString.Instance.Write(value.httpClientError, stream);
        }
    }
}