using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FilterListManagerSafeHandle : FFISafeHandle
    {
        public FilterListManagerSafeHandle()
            : base() { }

        public FilterListManagerSafeHandle(IntPtr pointer)
            : base(pointer) { }

        protected override bool ReleaseHandle()
        {
            UniffiHelpers.RustCall(
                (ref RustCallStatus status) =>
                {
                    UniFfiLib.uniffi_filter_list_manager_ffi_fn_free_filterlistmanager(
                        handle,
                        ref status
                    );
                }
            );
            return true;
        }
    }
}