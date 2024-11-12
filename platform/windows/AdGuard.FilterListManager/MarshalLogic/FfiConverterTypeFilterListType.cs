using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterListType : FfiConverterRustBuffer<FilterListType>
    {
        public static FfiConverterTypeFilterListType Instance = new FfiConverterTypeFilterListType();

        public override FilterListType Read(BigEndianStream stream)
        {
            var value = stream.ReadInt() - 1;
            if (Enum.IsDefined(typeof(FilterListType), value))
            {
                return (FilterListType)value;
            }

            throw new InternalException(
                $"invalid enum value '{value}' in FfiConverterTypeFilterListType.Read()"
            );
        }

        public override int AllocationSize(FilterListType value)
        {
            return 4;
        }

        public override void Write(FilterListType value, BigEndianStream stream)
        {
            stream.WriteInt((int)value + 1);
        }
    }
}