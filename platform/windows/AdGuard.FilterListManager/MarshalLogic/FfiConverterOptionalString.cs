namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalString : FfiConverterRustBuffer<string>
    {
        public static FfiConverterOptionalString INSTANCE = new FfiConverterOptionalString();

        public override string Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterString.INSTANCE.Read(stream);
        }

        public override int AllocationSize(string value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1 + FfiConverterString.INSTANCE.AllocationSize(value);
        }

        public override void Write(string value, BigEndianStream stream)
        {
            if (value == null)
            {
                stream.WriteByte(0);
            }
            else
            {
                stream.WriteByte(1);
                FfiConverterString.INSTANCE.Write(value, stream);
            }
        }
    }
}