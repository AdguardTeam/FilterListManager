namespace AdGuard.FilterListManager.RustInterface
{
    /// <summary>
    /// Discriminant for [`RustResponse`] result_data value
    /// </summary>
    internal enum RustResponseType : byte
    {
        /// <summary>
        /// Contains u8 pointer with size
        /// </summary>
        RustBuffer,
        
        /// <summary>
        /// Contains [`FLMHandle`]
        /// </summary>
        FLMHandlePointer
    }
}
