namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterListRules : FfiConverterRustBuffer<FilterListRules>
    {
        public static FfiConverterTypeFilterListRules INSTANCE = new FfiConverterTypeFilterListRules();

        public override FilterListRules Read(BigEndianStream stream)
        {
            return new FilterListRules(
                filterId: FfiConverterInt64.INSTANCE.Read(stream),
                rules: FfiConverterSequenceString.INSTANCE.Read(stream),
                disabledRules: FfiConverterSequenceString.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(FilterListRules value)
        {
            return FfiConverterInt64.INSTANCE.AllocationSize(value.filterId)
                   + FfiConverterSequenceString.INSTANCE.AllocationSize(value.rules)
                   + FfiConverterSequenceString.INSTANCE.AllocationSize(value.disabledRules);
        }

        public override void Write(FilterListRules value, BigEndianStream stream)
        {
            FfiConverterInt64.INSTANCE.Write(value.filterId, stream);
            FfiConverterSequenceString.INSTANCE.Write(value.rules, stream);
            FfiConverterSequenceString.INSTANCE.Write(value.disabledRules, stream);
        }
    }
}