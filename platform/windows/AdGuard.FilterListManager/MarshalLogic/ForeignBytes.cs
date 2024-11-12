using System;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManager.MarshalLogic
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct ForeignBytes
    {
        public int length;
        public IntPtr data;
    }
}