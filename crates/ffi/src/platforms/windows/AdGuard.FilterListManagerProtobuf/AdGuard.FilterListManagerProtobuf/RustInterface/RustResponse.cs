using System;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManagerProtobuf.RustInterface
{
    [StructLayout(LayoutKind.Sequential)]
    class RustResponse
    {
        public ulong result_data_len;
        public IntPtr result_data;
        public bool ffi_error;
        public RustResponseType discriminant;
    }
}

