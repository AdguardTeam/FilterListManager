using System;
using System.Runtime.InteropServices;
using AdGuard.FilterListManagerProtobuf.Utils;

namespace AdGuard.FilterListManagerProtobuf.RustInterface
{
    /// <summary>
    /// Basic class for establishing bridging between Rust and C# worlds
    /// </summary>
    static class ProtobufInterop
    {
        /// <summary>
        /// Getter for the set of [`FilterListManager`] constants
        /// </summary>
        /// <returns></returns>
        [DllImport(Constants.FLM_DLL_NAME)]
        internal static extern FLMConstants flm_get_constants();
        
        /// <summary>
        /// Makes default [`Configuration`] object as protobuf in [`RustResponse`]
        /// </summary>
        [DllImport(Constants.FLM_DLL_NAME)]
        internal static extern IntPtr flm_default_configuration_protobuf();

        /// <summary>
        /// Makes an FLM object and returns opaque pointer of [`FLMHandle`]
        /// </summary>
        [DllImport(Constants.FLM_DLL_NAME)]
        internal static extern IntPtr flm_init_protobuf(IntPtr pConfiguration, ulong configLength);
        
        /// <summary>
        /// Calls FLM method described as [`FFIMethod`] for object behind [`FLMHandle`]
        /// </summary>
        [DllImport(Constants.FLM_DLL_NAME)]
        internal static extern IntPtr flm_call_protobuf(IntPtr pHandle, FfiMethod ffiMethod, IntPtr pInputData, ulong inputDataLength);
        
        /// <summary>
        /// Frees memory of [`RustResponse`] objects and their data.
        /// NOTE: Actions for each discriminant are different.
        /// </summary>
        [DllImport(Constants.FLM_DLL_NAME)]
        internal static extern void flm_free_response(IntPtr pResponse);
        
        /// <summary>
        /// Drops [`FLMHandle`]
        /// </summary>
        [DllImport(Constants.FLM_DLL_NAME)]
        internal static extern void flm_free_handle(IntPtr pHandle);
    }
}
