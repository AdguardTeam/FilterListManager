namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalTypeUpdateResult : FfiConverterRustBuffer<UpdateResult>
    {
        public static FfiConverterOptionalTypeUpdateResult INSTANCE =
            new FfiConverterOptionalTypeUpdateResult();

        public override UpdateResult Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterTypeUpdateResult.INSTANCE.Read(stream);
        }

        public override int AllocationSize(UpdateResult value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1 + FfiConverterTypeUpdateResult.INSTANCE.AllocationSize(value);
        }

        public override void Write(UpdateResult value, BigEndianStream stream)
        {
            if (value == null)
            {
                stream.WriteByte(0);
            }
            else
            {
                stream.WriteByte(1);
                FfiConverterTypeUpdateResult.INSTANCE.Write(value, stream);
            }
        }
    }
}