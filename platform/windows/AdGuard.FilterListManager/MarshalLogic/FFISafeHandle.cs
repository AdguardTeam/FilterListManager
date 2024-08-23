using System;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public abstract class FFISafeHandle : SafeHandle
    {
        public FFISafeHandle()
            : base(new IntPtr(0), true) { }

        public FFISafeHandle(IntPtr pointer)
            : this()
        {
            SetHandle(pointer);
        }

        public override bool IsInvalid
        {
            get { return handle.ToInt64() == 0; }
        }

        // TODO(CS) this completely breaks any guarantees offered by SafeHandle.. Extracting
        // raw value from SafeHandle puts responsiblity on the consumer of this function to
        // ensure that SafeHandle outlives the stream, and anyone who might have read the raw
        // value from the stream and are holding onto it. Otherwise, the result might be a use
        // after free, or free while method calls are still in flight.
        //
        // This is also relevant for Kotlin.
        //
        public IntPtr DangerousGetRawFfiValue()
        {
            return handle;
        }
    }
}