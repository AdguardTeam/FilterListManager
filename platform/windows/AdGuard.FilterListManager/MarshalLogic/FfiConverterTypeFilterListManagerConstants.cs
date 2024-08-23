namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeFilterListManagerConstants
        : FfiConverterRustBuffer<FilterListManagerConstants>
    {
        public static FfiConverterTypeFilterListManagerConstants INSTANCE =
            new FfiConverterTypeFilterListManagerConstants();

        public override FilterListManagerConstants Read(BigEndianStream stream)
        {
            return new FilterListManagerConstants(
                userRulesId: FfiConverterInt64.INSTANCE.Read(stream),
                customGroupId: FfiConverterInt32.INSTANCE.Read(stream),
                specialGroupId: FfiConverterInt32.INSTANCE.Read(stream),
                smallestFilterId: FfiConverterInt64.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(FilterListManagerConstants value)
        {
            return FfiConverterInt64.INSTANCE.AllocationSize(value.userRulesId)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.customGroupId)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.specialGroupId)
                   + FfiConverterInt64.INSTANCE.AllocationSize(value.smallestFilterId);
        }

        public override void Write(FilterListManagerConstants value, BigEndianStream stream)
        {
            FfiConverterInt64.INSTANCE.Write(value.userRulesId, stream);
            FfiConverterInt32.INSTANCE.Write(value.customGroupId, stream);
            FfiConverterInt32.INSTANCE.Write(value.specialGroupId, stream);
            FfiConverterInt64.INSTANCE.Write(value.smallestFilterId, stream);
        }
    }
}
