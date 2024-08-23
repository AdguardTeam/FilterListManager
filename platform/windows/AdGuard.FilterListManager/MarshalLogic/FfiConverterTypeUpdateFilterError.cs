namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeUpdateFilterError : FfiConverterRustBuffer<UpdateFilterError>
    {
        public static FfiConverterTypeUpdateFilterError INSTANCE =
            new FfiConverterTypeUpdateFilterError();

        public override UpdateFilterError Read(BigEndianStream stream)
        {
            return new UpdateFilterError(
                filterId: FfiConverterInt64.INSTANCE.Read(stream),
                message: FfiConverterString.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(UpdateFilterError value)
        {
            return FfiConverterInt64.INSTANCE.AllocationSize(value.filterId)
                   + FfiConverterString.INSTANCE.AllocationSize(value.message);
        }

        public override void Write(UpdateFilterError value, BigEndianStream stream)
        {
            FfiConverterInt64.INSTANCE.Write(value.filterId, stream);
            FfiConverterString.INSTANCE.Write(value.message, stream);
        }
    }
}