namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalTypeFilterListRules : FfiConverterRustBuffer<FilterListRules>
    {
        public static FfiConverterOptionalTypeFilterListRules Instance =
            new FfiConverterOptionalTypeFilterListRules();

        public override FilterListRules Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterTypeFilterListRules.Instance.Read(stream);
        }

        public override int AllocationSize(FilterListRules value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1
                   + FfiConverterTypeFilterListRules.Instance.AllocationSize(value);
        }

        public override void Write(FilterListRules value, BigEndianStream stream)
        {
            if (value == null)
            {
                stream.WriteByte(0);
            }
            else
            {
                stream.WriteByte(1);
                FfiConverterTypeFilterListRules.Instance.Write(value, stream);
            }
        }
    }
}