using System;
using AdGuard.FilterListManagerProtobuf.RustInterface;

namespace AdGuard.FilterListManagerProtobuf.Api
{
    /// <summary>
    /// Rust response result
    /// </summary>
    internal class RustResponseResult
    {
        /// <summary>
        /// Handle pointer, valid and make sense only if <see cref="Discriminant"/>
        /// equals to <see cref="RustResponseType.FLMHandlePointer"/>
        /// </summary>
        internal IntPtr HandlePointer { get; set; }
        
        /// <summary>
        /// Handle pointer, valid and make sense only if <see cref="Discriminant"/>
        /// equals to <see cref="RustResponseType.RustBuffer"/>
        /// </summary>
        internal byte[] Buffer { get; set; }
        
        /// <summary>
        /// Discriminant, which specifies whether <see cref="HandlePointer"/> or <see cref="Buffer"/> is valid and make sense
        /// </summary>
        internal RustResponseType Discriminant { get; set; }
    }
}