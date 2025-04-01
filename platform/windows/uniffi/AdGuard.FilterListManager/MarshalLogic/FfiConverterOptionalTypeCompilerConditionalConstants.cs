namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalTypeCompilerConditionalConstants : FfiConverterRustBuffer<CompilerConditionalConstants>
    {
        public static FfiConverterOptionalTypeCompilerConditionalConstants Instance =
            new FfiConverterOptionalTypeCompilerConditionalConstants();

        public override CompilerConditionalConstants Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterTypeCompilerConditionalConstants.Instance.Read(stream);
        }

        public override int AllocationSize(CompilerConditionalConstants value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1
                   + FfiConverterTypeCompilerConditionalConstants.Instance.AllocationSize(value);
        }

        public override void Write(CompilerConditionalConstants value, BigEndianStream stream)
        {
            if (value == null)
            {
                stream.WriteByte(0);
            }
            else
            {
                stream.WriteByte(1);
                FfiConverterTypeCompilerConditionalConstants.Instance.Write(value, stream);
            }
        }
    }
}