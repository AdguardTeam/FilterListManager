namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalTypeFullFilterList : FfiConverterRustBuffer<FullFilterList>
    {
        public static FfiConverterOptionalTypeFullFilterList Instance =
            new FfiConverterOptionalTypeFullFilterList();

        public override FullFilterList Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterTypeFullFilterList.Instance.Read(stream);
        }

        public override int AllocationSize(FullFilterList value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1
                   + FfiConverterTypeFullFilterList.Instance.AllocationSize(value);
        }

        public override void Write(FullFilterList value, BigEndianStream stream)
        {
            if (value == null)
            {
                stream.WriteByte(0);
            }
            else
            {
                stream.WriteByte(1);
                FfiConverterTypeFullFilterList.Instance.Write(value, stream);
            }
        }
    }
}