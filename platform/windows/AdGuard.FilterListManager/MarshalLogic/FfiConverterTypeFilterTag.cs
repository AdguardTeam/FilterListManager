namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterTag : FfiConverterRustBuffer<FilterTag>
    {
        public static FfiConverterTypeFilterTag Instance = new FfiConverterTypeFilterTag();

        public override FilterTag Read(BigEndianStream stream)
        {
            return new FilterTag(
                id: FfiConverterInt32.Instance.Read(stream),
                keyword: FfiConverterString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FilterTag value)
        {
            return FfiConverterInt32.Instance.AllocationSize(value.id)
                   + FfiConverterString.Instance.AllocationSize(value.keyword);
        }

        public override void Write(FilterTag value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.id, stream);
            FfiConverterString.Instance.Write(value.keyword, stream);
        }
    }
}