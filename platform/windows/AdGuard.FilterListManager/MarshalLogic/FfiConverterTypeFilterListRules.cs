namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterListRules : FfiConverterRustBuffer<FilterListRules>
    {
        public static FfiConverterTypeFilterListRules Instance = new FfiConverterTypeFilterListRules();

        public override FilterListRules Read(BigEndianStream stream)
        {
            return new FilterListRules(
                filterId: FfiConverterInt32.Instance.Read(stream),
                rules: FfiConverterSequenceString.Instance.Read(stream),
                disabledRules: FfiConverterSequenceString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FilterListRules value)
        {
            return FfiConverterInt32.Instance.AllocationSize(value.filterId)
                   + FfiConverterSequenceString.Instance.AllocationSize(value.rules)
                   + FfiConverterSequenceString.Instance.AllocationSize(value.disabledRules);
        }

        public override void Write(FilterListRules value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.filterId, stream);
            FfiConverterSequenceString.Instance.Write(value.rules, stream);
            FfiConverterSequenceString.Instance.Write(value.disabledRules, stream);
        }
    }
}