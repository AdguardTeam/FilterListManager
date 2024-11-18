﻿namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FfiConverterTypeFilterListManagerConstants
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
            return FfiConverterInt64.Instance.AllocationSize(value.UserRulesId)
                   + FfiConverterInt32.Instance.AllocationSize(value.CustomGroupId)
                   + FfiConverterInt32.Instance.AllocationSize(value.SpecialGroupId)
                   + FfiConverterInt64.Instance.AllocationSize(value.SmallestFilterId);
        }

        public override void Write(FilterListManagerConstants value, BigEndianStream stream)
        {
            FfiConverterInt64.Instance.Write(value.UserRulesId, stream);
            FfiConverterInt32.Instance.Write(value.CustomGroupId, stream);
            FfiConverterInt32.Instance.Write(value.SpecialGroupId, stream);
            FfiConverterInt64.Instance.Write(value.SmallestFilterId, stream);
        }
    }
}