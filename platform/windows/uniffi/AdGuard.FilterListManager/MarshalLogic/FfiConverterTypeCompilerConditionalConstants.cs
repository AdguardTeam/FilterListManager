namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeCompilerConditionalConstants : FfiConverterRustBuffer<CompilerConditionalConstants>
    {
        public static FfiConverterTypeCompilerConditionalConstants Instance = new FfiConverterTypeCompilerConditionalConstants();

        public override CompilerConditionalConstants Read(BigEndianStream stream)
        {
            return new CompilerConditionalConstants(
                compilerConditionalConstants: FfiConverterSequenceString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(CompilerConditionalConstants value)
        {
            return FfiConverterSequenceString.Instance.AllocationSize(
                value.CompilerConditionalConstants
            );
        }

        public override void Write(CompilerConditionalConstants value, BigEndianStream stream)
        {
            FfiConverterSequenceString.Instance.Write(
                value.CompilerConditionalConstants,
                stream
            );
        }
    }
}