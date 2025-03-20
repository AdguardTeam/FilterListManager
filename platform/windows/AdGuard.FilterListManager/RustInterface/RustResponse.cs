using System;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManager.RustInterface
{
    /// <summary>
    /// Container for rust-formed responses into external world
    /// UNSAFE: You must manually control the release of any types folded into the “response”
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    internal struct RustResponse
    {
        /**
         * Bytes count
         * UNSAFE: You should put here the real data length, even for pointers
         */
        internal UIntPtr ResultDataLen;
        
        /**
        * The real allocated data length
        * UNSAFE: You should put here the real data length, even for pointers
        */
        internal UIntPtr ResultDataCapacity;
        
        /**
         * UNSAFE: There can be many different pointer types
         */
        internal IntPtr PResultData;
        
        /**
         * Special response case:
         * If request or response have failed, try to send [`AGOuterError::Other`] error with the explanation
         * See: [`build_rust_response_error`]
         */
        [MarshalAs(UnmanagedType.U1)]
        internal bool FfiError;
        
        /**
         * Data type discriminant
         */
        [MarshalAs(UnmanagedType.U1)]
        internal RustResponseType Discriminant;
    }
}

