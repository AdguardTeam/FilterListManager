using System.Runtime.InteropServices;

namespace AdGuard.FilterListManager.MarshalLogic
{
    [StructLayout(LayoutKind.Sequential)]
    struct RustCallStatus
    {
        public sbyte code;
        public RustBuffer error_buf;

        public bool IsSuccess()
        {
            return code == 0;
        }

        public bool IsError()
        {
            return code == 1;
        }

        public bool IsPanic()
        {
            return code == 2;
        }
    }
}