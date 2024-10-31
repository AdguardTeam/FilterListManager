namespace AdGuard.FilterListManager.MarshalLogic
{
    public abstract class FfiConverterRustBuffer<CsType> : FfiConverter<CsType, RustBuffer>
    {
        public override CsType Lift(RustBuffer value)
        {
            return LiftFromRustBuffer(value);
        }

        public override RustBuffer Lower(CsType value)
        {
            return LowerIntoRustBuffer(value);
        }
    }
}