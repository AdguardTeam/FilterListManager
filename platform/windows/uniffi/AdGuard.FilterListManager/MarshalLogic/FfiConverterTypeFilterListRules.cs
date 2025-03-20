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
                disabledRules: FfiConverterSequenceString.Instance.Read(stream),
                rulesCount: FfiConverterInt32.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FilterListRules value)
        {
            return FfiConverterInt32.Instance.AllocationSize(value.FilterId)
                   + FfiConverterSequenceString.Instance.AllocationSize(value.Rules)
                   + FfiConverterSequenceString.Instance.AllocationSize(value.DisabledRules)
                   + FfiConverterInt32.Instance.AllocationSize(value.RulesCount);
        }

        public override void Write(FilterListRules value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.FilterId, stream);
            FfiConverterSequenceString.Instance.Write(value.Rules, stream);
            FfiConverterSequenceString.Instance.Write(value.DisabledRules, stream);
            FfiConverterInt32.Instance.Write(value.RulesCount, stream);
        }
    }
}