using System;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManagerProtobuf.RustInterface
{
    [StructLayout(LayoutKind.Sequential)]
    struct RustResponse
    {
        public ulong ResultDataLen { get; set; }
        public IntPtr ResultData { get; set; }
        public bool FfiError { get; set; }
        public RustResponseType Discriminant { get; set; }
    }
}

