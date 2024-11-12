namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterBoolean : FfiConverter<bool, sbyte>
    {
        public static FfiConverterBoolean Instance = new FfiConverterBoolean();

        public override bool Lift(sbyte value)
        {
            return value != 0;
        }

        public override bool Read(BigEndianStream stream)
        {
            return Lift(stream.ReadSByte());
        }

        public override sbyte Lower(bool value)
        {
            return value ? (sbyte)1 : (sbyte)0;
        }

        public override int AllocationSize(bool value)
        {
            return 1;
        }

        public override void Write(bool value, BigEndianStream stream)
        {
            stream.WriteSByte(Lower(value));
        }
    }
}