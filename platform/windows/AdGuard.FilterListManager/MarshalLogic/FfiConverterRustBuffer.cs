namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// C# types - <see cref="RustBuffer"/> converter
    /// </summary>
    /// <typeparam name="TCsType">The type of the cs type.</typeparam>
    public abstract class FfiConverterRustBuffer<TCsType> : FfiConverter<TCsType, RustBuffer>
    {
        /// <summary>
        /// Convert an FFI type to a C# type
        /// </summary>
        /// <param name="value">The value.</param>
        public override TCsType Lift(RustBuffer value)
        {
            return LiftFromRustBuffer(value);
        }

        /// <summary>
        /// Convert C# type to an FFI type
        /// </summary>
        /// <param name="value">The value.</param>
        public override RustBuffer Lower(TCsType value)
        {
            return LowerIntoRustBuffer(value);
        }
    }
}