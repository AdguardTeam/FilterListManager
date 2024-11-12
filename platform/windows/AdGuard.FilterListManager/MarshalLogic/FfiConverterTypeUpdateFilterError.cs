namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeUpdateFilterError : FfiConverterRustBuffer<UpdateFilterError>
    {
        public static FfiConverterTypeUpdateFilterError Instance =
            new FfiConverterTypeUpdateFilterError();

        public override UpdateFilterError Read(BigEndianStream stream)
        {
            return new UpdateFilterError(
                filterId: FfiConverterInt64.Instance.Read(stream),
                message: FfiConverterString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(UpdateFilterError value)
        {
            return FfiConverterInt64.Instance.AllocationSize(value.filterId)
                   + FfiConverterString.Instance.AllocationSize(value.message);
        }

        public override void Write(UpdateFilterError value, BigEndianStream stream)
        {
            FfiConverterInt64.Instance.Write(value.filterId, stream);
            FfiConverterString.Instance.Write(value.message, stream);
        }
    }
}