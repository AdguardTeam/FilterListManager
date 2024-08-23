namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterInt64 : FfiConverter<long, long>
    {
        public static FfiConverterInt64 INSTANCE = new FfiConverterInt64();

        public override long Lift(long value)
        {
            return value;
        }

        public override long Read(BigEndianStream stream)
        {
            return stream.ReadLong();
        }

        public override long Lower(long value)
        {
            return value;
        }

        public override int AllocationSize(long value)
        {
            return 8;
        }

        public override void Write(long value, BigEndianStream stream)
        {
            stream.WriteLong(value);
        }
    }
}
