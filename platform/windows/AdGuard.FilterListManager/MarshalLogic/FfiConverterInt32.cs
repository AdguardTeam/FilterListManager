namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterInt32 : FfiConverter<int, int>
    {
        public static FfiConverterInt32 INSTANCE = new FfiConverterInt32();

        public override int Lift(int value)
        {
            return value;
        }

        public override int Read(BigEndianStream stream)
        {
            return stream.ReadInt();
        }

        public override int Lower(int value)
        {
            return value;
        }

        public override int AllocationSize(int value)
        {
            return 4;
        }

        public override void Write(int value, BigEndianStream stream)
        {
            stream.WriteInt(value);
        }
    }
}