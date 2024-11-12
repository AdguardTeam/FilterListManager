namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeFilterListManagerConstants
        : FfiConverterRustBuffer<FilterListManagerConstants>
    {
        public static FfiConverterTypeFilterListManagerConstants Instance =
            new FfiConverterTypeFilterListManagerConstants();

        public override FilterListManagerConstants Read(BigEndianStream stream)
        {
            return new FilterListManagerConstants(
                userRulesId: FfiConverterInt64.Instance.Read(stream),
                customGroupId: FfiConverterInt32.Instance.Read(stream),
                specialGroupId: FfiConverterInt32.Instance.Read(stream),
                smallestFilterId: FfiConverterInt64.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FilterListManagerConstants value)
        {
            return FfiConverterInt64.Instance.AllocationSize(value.userRulesId)
                   + FfiConverterInt32.Instance.AllocationSize(value.customGroupId)
                   + FfiConverterInt32.Instance.AllocationSize(value.specialGroupId)
                   + FfiConverterInt64.Instance.AllocationSize(value.smallestFilterId);
        }

        public override void Write(FilterListManagerConstants value, BigEndianStream stream)
        {
            FfiConverterInt64.Instance.Write(value.userRulesId, stream);
            FfiConverterInt32.Instance.Write(value.customGroupId, stream);
            FfiConverterInt32.Instance.Write(value.specialGroupId, stream);
            FfiConverterInt64.Instance.Write(value.smallestFilterId, stream);
        }
    }
}
