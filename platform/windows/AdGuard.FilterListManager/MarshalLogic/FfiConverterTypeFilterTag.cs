namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterTag : FfiConverterRustBuffer<FilterTag>
    {
        public static FfiConverterTypeFilterTag INSTANCE = new FfiConverterTypeFilterTag();

        public override FilterTag Read(BigEndianStream stream)
        {
            return new FilterTag(
                id: FfiConverterInt32.INSTANCE.Read(stream),
                keyword: FfiConverterString.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(FilterTag value)
        {
            return FfiConverterInt32.INSTANCE.AllocationSize(value.id)
                   + FfiConverterString.INSTANCE.AllocationSize(value.keyword);
        }

        public override void Write(FilterTag value, BigEndianStream stream)
        {
            FfiConverterInt32.INSTANCE.Write(value.id, stream);
            FfiConverterString.INSTANCE.Write(value.keyword, stream);
        }
    }
}