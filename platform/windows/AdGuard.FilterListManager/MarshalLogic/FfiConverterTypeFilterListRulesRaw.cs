namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// List(FilterListRulesRaw) to <see cref="RustBuffer"/> converter
    /// </summary>
    public class FfiConverterTypeFilterListRulesRaw : FfiConverterRustBuffer<FilterListRulesRaw>
    {
        public static FfiConverterTypeFilterListRulesRaw Instance = new FfiConverterTypeFilterListRulesRaw();

        ///  <inheritdoc/>
        public override FilterListRulesRaw Read(BigEndianStream stream)
        {
            return new FilterListRulesRaw(
                filterId: FfiConverterInt32.Instance.Read(stream),
                rules: FfiConverterString.Instance.Read(stream),
                disabledRules: FfiConverterString.Instance.Read(stream)
            );
        }

        ///  <inheritdoc/>
        public override int AllocationSize(FilterListRulesRaw value)
        {
            return
                FfiConverterInt32.Instance.AllocationSize(value.FilterId) +
                FfiConverterString.Instance.AllocationSize(value.Rules) +
                FfiConverterString.Instance.AllocationSize(value.DisabledRules);
        }

        ///  <inheritdoc/>
        public override void Write(FilterListRulesRaw value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.FilterId, stream);
            FfiConverterString.Instance.Write(value.Rules, stream);
            FfiConverterString.Instance.Write(value.DisabledRules, stream);
        }
    }
}
