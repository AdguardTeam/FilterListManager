using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public abstract class FfiConverter<CsType, FfiType>
    {
        // Convert an FFI type to a C# type
        public abstract CsType Lift(FfiType value);

        // Convert C# type to an FFI type
        public abstract FfiType Lower(CsType value);

        // Read a C# type from a `ByteBuffer`
        public abstract CsType Read(BigEndianStream stream);

        // Calculate bytes to allocate when creating a `RustBuffer`
        //
        // This must return at least as many bytes as the write() function will
        // write. It can return more bytes than needed, for example when writing
        // Strings we can't know the exact bytes needed until we the UTF-8
        // encoding, so we pessimistically allocate the largest size possible (3
        // bytes per codepoint).  Allocating extra bytes is not really a big deal
        // because the `RustBuffer` is short-lived.
        public abstract int AllocationSize(CsType value);

        // Write a C# type to a `ByteBuffer`
        public abstract void Write(CsType value, BigEndianStream stream);

        // Lower a value into a `RustBuffer`
        //
        // This method lowers a value into a `RustBuffer` rather than the normal
        // FfiType.  It's used by the callback interface code.  Callback interface
        // returns are always serialized into a `RustBuffer` regardless of their
        // normal FFI type.
        public RustBuffer LowerIntoRustBuffer(CsType value)
        {
            var rbuf = RustBuffer.Alloc(AllocationSize(value));
            try
            {
                var stream = rbuf.AsWriteableStream();
                Write(value, stream);
                rbuf.len = Convert.ToInt32(stream.Position);
                return rbuf;
            }
            catch
            {
                RustBuffer.Free(rbuf);
                throw;
            }
        }

        // Lift a value from a `RustBuffer`.
        //
        // This here mostly because of the symmetry with `lowerIntoRustBuffer()`.
        // It's currently only used by the `FfiConverterRustBuffer` class below.
        protected CsType LiftFromRustBuffer(RustBuffer rbuf)
        {
            var stream = rbuf.AsStream();
            try
            {
                var item = Read(stream);
                if (stream.HasRemaining())
                {
                    throw new InternalException(
                        "junk remaining in buffer after lifting, something is very wrong!!"
                    );
                }
                return item;
            }
            finally
            {
                RustBuffer.Free(rbuf);
            }
        }
    }
}