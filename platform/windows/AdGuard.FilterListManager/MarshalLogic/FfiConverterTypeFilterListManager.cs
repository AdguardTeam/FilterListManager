using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeFilterListManager
        : FfiConverter<FilterListManager, FilterListManagerSafeHandle>
    {
        public static FfiConverterTypeFilterListManager INSTANCE =
            new FfiConverterTypeFilterListManager();

        public override FilterListManagerSafeHandle Lower(FilterListManager value)
        {
            return value.GetHandle();
        }

        public override FilterListManager Lift(FilterListManagerSafeHandle value)
        {
            return new FilterListManager(value);
        }

        public override FilterListManager Read(BigEndianStream stream)
        {
            return Lift(new FilterListManagerSafeHandle(new IntPtr(stream.ReadLong())));
        }

        public override int AllocationSize(FilterListManager value)
        {
            return 8;
        }

        public override void Write(FilterListManager value, BigEndianStream stream)
        {
            stream.WriteLong(Lower(value).DangerousGetRawFfiValue().ToInt64());
        }
    }

}
