using System;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Base implementation of <see cref="SafeHandle"/>
    /// </summary>
    /// <seealso cref="SafeHandle" />
    public abstract class FfiSafeHandle : SafeHandle
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FfiSafeHandle"/> class.
        /// </summary>
        protected FfiSafeHandle() : base(new IntPtr(0), true) { }

        /// <summary>
        /// Initializes a new instance of the <see cref="FfiSafeHandle"/> class.
        /// </summary>
        /// <param name="pointer">The pointer.</param>
        protected FfiSafeHandle(IntPtr pointer) : this()
        {
            SetHandle(pointer);
        }

        /// <summary>
        /// When overridden in a derived class, gets a value indicating whether the handle value is invalid.
        /// </summary>
        /// <PermissionSet>
        ///   <IPermission class="System.Security.Permissions.SecurityPermission, mscorlib, Version=2.0.3600.0, Culture=neutral, PublicKeyToken=b77a5c561934e089" version="1" Flags="UnmanagedCode" />
        /// </PermissionSet>
        public override bool IsInvalid
        {
            get { return handle.ToInt64() == 0; }
        }

        /// <summary>
        /// Gets the get raw FFI value.
        /// TODO(CS) this completely breaks any guarantees offered by SafeHandle. Extracting
        /// raw value from SafeHandle puts responsibility on the consumer of this function to
        /// ensure that SafeHandle outlives the stream, and anyone who might have read the raw
        /// value from the stream and are holding onto it. Otherwise, the result might be a use
        /// after free, or free while method calls are still in flight.
        ///
        /// This is also relevant for Kotlin.
        /// </summary>
        public IntPtr DangerousGetRawFfiValue()
        {
            return handle;
        }
    }
}