namespace AdGuard.FilterListManagerProtobuf.RustInterface
{
    /// <summary>
    /// Discriminant for [`RustResponse`] result_data value
    /// </summary>
    enum RustResponseType
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
