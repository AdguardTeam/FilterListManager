using System;
using System.Runtime.InteropServices;
using AdGuard.FilterListManagerProtobuf.Utils;

namespace AdGuard.FilterListManagerProtobuf.RustInterface
{
    static class ProtobufInterop
    {
        [DllImport(Constants.FLM_DLL_IMPORT_NAME)]
        internal static extern IntPtr flm_default_configuration_protobuf();

        [DllImport(Constants.FLM_DLL_IMPORT_NAME)]
        internal static extern IntPtr flm_init_protobuf(IntPtr configuration, ulong configLength);

        [DllImport(Constants.FLM_DLL_IMPORT_NAME)]
        internal static extern IntPtr flm_call_protobuf(IntPtr handle, FFIMethod method, IntPtr inputData, ulong inputDataLength);

        [DllImport(Constants.FLM_DLL_IMPORT_NAME)]
        internal static extern void flm_free_response(IntPtr response);

        [DllImport(Constants.FLM_DLL_IMPORT_NAME)]
        internal static extern void flm_free_handle(IntPtr handle);
    }
}
