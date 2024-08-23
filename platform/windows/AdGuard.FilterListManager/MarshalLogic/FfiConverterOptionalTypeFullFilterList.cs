namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalTypeFullFilterList : FfiConverterRustBuffer<FullFilterList>
    {
        public static FfiConverterOptionalTypeFullFilterList INSTANCE =
            new FfiConverterOptionalTypeFullFilterList();

        public override FullFilterList Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterTypeFullFilterList.INSTANCE.Read(stream);
        }

        public override int AllocationSize(FullFilterList value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1
                   + FfiConverterTypeFullFilterList.INSTANCE.AllocationSize(value);
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
                FfiConverterTypeFullFilterList.INSTANCE.Write(value, stream);
            }
        }
    }
}