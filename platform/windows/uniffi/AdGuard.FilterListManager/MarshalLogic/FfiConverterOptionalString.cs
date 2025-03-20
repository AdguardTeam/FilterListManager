namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalString : FfiConverterRustBuffer<string>
    {
        public static FfiConverterOptionalString Instance = new FfiConverterOptionalString();

        public override string Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterString.Instance.Read(stream);
        }

        public override int AllocationSize(string value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1 + FfiConverterString.Instance.AllocationSize(value);
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
                FfiConverterString.Instance.Write(value, stream);
            }
        }
    }
}