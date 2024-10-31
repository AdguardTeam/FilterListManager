namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FfiConverterTypeFilterListRulesRaw : FfiConverterRustBuffer<FilterListRulesRaw>
    {
        public static FfiConverterTypeFilterListRulesRaw Instance = new FfiConverterTypeFilterListRulesRaw();

        public override FilterListRulesRaw Read(BigEndianStream stream)
        {
            return new FilterListRulesRaw(
                @filterId: FfiConverterInt64.INSTANCE.Read(stream),
                @rules: FfiConverterString.INSTANCE.Read(stream),
                @disabledRules: FfiConverterString.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(FilterListRulesRaw value)
        {
            return
                FfiConverterInt64.INSTANCE.AllocationSize(value.FilterId) +
                FfiConverterString.INSTANCE.AllocationSize(value.Rules) +
                FfiConverterString.INSTANCE.AllocationSize(value.DisabledRules);
        }

        public override void Write(FilterListRulesRaw value, BigEndianStream stream)
        {
            FfiConverterInt64.INSTANCE.Write(value.FilterId, stream);
            FfiConverterString.INSTANCE.Write(value.Rules, stream);
            FfiConverterString.INSTANCE.Write(value.DisabledRules, stream);
        }
    }
}
