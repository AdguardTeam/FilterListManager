namespace AdGuard.FilterListManager.MarshalLogic
{ 
    /// <summary>
    /// List(DisabledRulesRaw) to <see cref="RustBuffer"/> converter
    /// </summary>
    public class FfiConverterTypeDisabledRulesRaw : FfiConverterRustBuffer<DisabledRulesRaw>
    {
        public static FfiConverterTypeDisabledRulesRaw Instance = new FfiConverterTypeDisabledRulesRaw();

        /// <inheritdoc/>
        public override DisabledRulesRaw Read(BigEndianStream stream)
        {
            return new DisabledRulesRaw(
                filterId: FfiConverterInt32.Instance.Read(stream),
                text: FfiConverterString.Instance.Read(stream)
            );
        }

        /// <inheritdoc/>
        public override int AllocationSize(DisabledRulesRaw value)
        {
            return
                FfiConverterInt32.Instance.AllocationSize(value.FilterId) +
                FfiConverterString.Instance.AllocationSize(value.Text);
        }

        /// <inheritdoc/>
        public override void Write(DisabledRulesRaw value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.FilterId, stream);
            FfiConverterString.Instance.Write(value.Text, stream);
        }
    }
}