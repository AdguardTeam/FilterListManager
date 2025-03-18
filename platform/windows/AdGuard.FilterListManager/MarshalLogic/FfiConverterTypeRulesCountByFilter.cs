namespace AdGuard.FilterListManager.MarshalLogic
{ 
    /// <summary>
    /// List(RulesCountByFilter) to <see cref="RustBuffer"/> converter
    /// </summary>
    public class FfiConverterTypeRulesCountByFilter : FfiConverterRustBuffer<RulesCountByFilter>
    {
        public static FfiConverterTypeRulesCountByFilter Instance = new FfiConverterTypeRulesCountByFilter();

        /// <inheritdoc/>
        public override RulesCountByFilter Read(BigEndianStream stream)
        {
            return new RulesCountByFilter(
                filterId: FfiConverterInt32.Instance.Read(stream),
                rulesCount: FfiConverterString.Instance.Read(stream)
            );
        }

        /// <inheritdoc/>
        public override int AllocationSize(RulesCountByFilter value)
        {
            return
                FfiConverterInt32.Instance.AllocationSize(value.FilterId)
                + FfiConverterInt32.Instance.AllocationSize(value.RulesCount);
        }

        /// <inheritdoc/>
        public override void Write(RulesCountByFilter value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.FilterId, stream);
            FfiConverterInt32.Instance.Write(value.RulesCount, stream);
        }
    }
}