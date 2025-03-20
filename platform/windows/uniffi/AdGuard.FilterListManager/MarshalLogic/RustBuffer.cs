using System;
using System.IO;
using System.Runtime.InteropServices;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// This is a helper for safely working with byte buffers returned from the Rust code.
    /// A rust-owned buffer is represented by its capacity, its current length, and a
    /// pointer to the underlying data.
    ///
    /// This is a helper for safely passing byte references into the rust code.
    /// It's not actually used at the moment, because there aren't many things that you
    /// can take a direct pointer to managed memory, and if we're going to copy something
    /// then we might as well copy it into a `RustBuffer`. But it's here for API
    /// completeness.

    /// The FfiConverter interface handles converter types to and from the FFI
    ///
    /// All implementing objects should be public to support external types.  When a
    /// type is external we need to import it's FfiConverter.

    /// FfiConverter that uses `RustBuffer` as the FfiType

    /// A handful of classes and functions to support the generated data structures.
    /// This would be a good candidate for isolating in its own ffi-support lib.
    /// Error runtime.

    /// Base class for all uniffi exceptions

    /// Each top-level error class has a companion object that can lift the error from the call status's rust buffer

    /// CallStatusErrorHandler implementation for times when we don't expect a CALL_ERROR

    /// Helpers for calling Rust
    /// In practice we usually need to be synchronized to call this safely, so it doesn't
    /// synchronize itself

    /// Big endian streams are not yet available in dotnet :'(
    /// https://github.com/dotnet/runtime/issues/26904

    /// Contains loading, initialization code,
    /// and the FFI Function declarations in a com.sun.jna.Library.


    /// This is an implementation detail which will be called publicly by the public API.

    /// Public interface members begin here.

    /// `SafeHandle` implements the semantics outlined below, i.e. its thread safe, and the dispose
    /// method will only be called once, once all outstanding native calls have completed.
    /// https://github.com/mozilla/uniffi-rs/blob/0dc031132d9493ca812c3af6e7dd60ad2ea95bf0/uniffi_bindgen/src/bindings/kotlin/templates/ObjectRuntime.kt#L31
    /// https://learn.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.criticalhandle
    ///
    /// 
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct RustBuffer
    {
        public int capacity;
        public int len;
        public IntPtr data;

        /// <summary>
        /// Allocates the specified size in memory.
        /// </summary>
        /// <param name="size">The size.</param>
        /// <returns>Allocated rust buffer</returns>
        public static RustBuffer Alloc(int size)
        {
            return UniffiHelpers.RustCall(
                (ref RustCallStatus status) =>
                {
                    var buffer = UniFfiLib.ffi_filter_list_manager_ffi_rustbuffer_alloc(
                        size,
                        ref status
                    );
                    if (buffer.data == IntPtr.Zero)
                    {
                        throw new AllocationException(
                            $"RustBuffer.Alloc() returned null data pointer (size={size})"
                        );
                    }
                    return buffer;
                }
            );
        }

        /// <summary>
        /// Frees the memory of the specified rust buffer.
        /// </summary>
        /// <param name="buffer">The buffer.</param>
        public static void Free(RustBuffer buffer)
        {
            UniffiHelpers.RustCall(
                (ref RustCallStatus status) =>
                {
                    UniFfiLib.ffi_filter_list_manager_ffi_rustbuffer_free(buffer, ref status);
                }
            );
        }

        internal static BigEndianStream MemoryStream(IntPtr data, int length)
        {
            unsafe
            {
                return new BigEndianStream(new UnmanagedMemoryStream((byte*)data.ToPointer(), length));
            }
        }

        internal BigEndianStream AsStream()
        {
            unsafe
            {
                return new BigEndianStream(new UnmanagedMemoryStream((byte*)data.ToPointer(), len));
            }
        }

        internal BigEndianStream AsWriteableStream()
        {
            unsafe
            {
                return new BigEndianStream(
                    new UnmanagedMemoryStream(
                        (byte*)data.ToPointer(),
                        capacity,
                        capacity,
                        FileAccess.Write
                    )
                );
            }
        }
    }
}
