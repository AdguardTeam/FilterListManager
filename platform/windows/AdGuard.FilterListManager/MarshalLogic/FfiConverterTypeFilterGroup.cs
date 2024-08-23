namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterGroup : FfiConverterRustBuffer<FilterGroup>
    {
        public static FfiConverterTypeFilterGroup INSTANCE = new FfiConverterTypeFilterGroup();

        public override FilterGroup Read(BigEndianStream stream)
        {
            return new FilterGroup(
                id: FfiConverterInt32.INSTANCE.Read(stream),
                name: FfiConverterString.INSTANCE.Read(stream),
                displayNumber: FfiConverterInt32.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(FilterGroup value)
        {
            return FfiConverterInt32.INSTANCE.AllocationSize(value.id)
                   + FfiConverterString.INSTANCE.AllocationSize(value.name)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.displayNumber);
        }

        public override void Write(FilterGroup value, BigEndianStream stream)
        {
            FfiConverterInt32.INSTANCE.Write(value.id, stream);
            FfiConverterString.INSTANCE.Write(value.name, stream);
            FfiConverterInt32.INSTANCE.Write(value.displayNumber, stream);
        }
    }
}