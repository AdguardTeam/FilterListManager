using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Main implementation of <see cref="FfiSafeHandle"/>
    /// </summary>
    /// <seealso cref="FfiSafeHandle" />
    public class FilterListManagerSafeHandle : FfiSafeHandle
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListManagerSafeHandle"/> class.
        /// </summary>
        public FilterListManagerSafeHandle()
            : base() { }

        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListManagerSafeHandle"/> class.
        /// </summary>
        /// <param name="pointer">The pointer.</param>
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