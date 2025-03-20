namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterGroup : FfiConverterRustBuffer<FilterGroup>
    {
        public static FfiConverterTypeFilterGroup Instance = new FfiConverterTypeFilterGroup();

        public override FilterGroup Read(BigEndianStream stream)
        {
            return new FilterGroup(
                id: FfiConverterInt32.Instance.Read(stream),
                name: FfiConverterString.Instance.Read(stream),
                displayNumber: FfiConverterInt32.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FilterGroup value)
        {
            return FfiConverterInt32.Instance.AllocationSize(value.id)
                   + FfiConverterString.Instance.AllocationSize(value.name)
                   + FfiConverterInt32.Instance.AllocationSize(value.displayNumber);
        }

        public override void Write(FilterGroup value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.id, stream);
            FfiConverterString.Instance.Write(value.name, stream);
            FfiConverterInt32.Instance.Write(value.displayNumber, stream);
        }
    }
}