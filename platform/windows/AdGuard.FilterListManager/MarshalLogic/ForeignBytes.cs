using System;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManager.MarshalLogic
{
    [StructLayout(LayoutKind.Sequential)]
    public struct ForeignBytes
    {
        public int length;
        public IntPtr data;
    }
}