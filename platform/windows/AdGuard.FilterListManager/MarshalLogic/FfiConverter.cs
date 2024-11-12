using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// C# types - FFI types converter
    /// </summary>
    /// <typeparam name="TCsType">The type of the s type.</typeparam>
    /// <typeparam name="TFfiType">The type of the fi type.</typeparam>
    public abstract class FfiConverter<TCsType, TFfiType>
    {
        /// <summary>
        /// Convert an FFI type to a C# type
        /// </summary>
        /// <param name="value">The value.</param>
        /// <returns></returns>
        public abstract TCsType Lift(TFfiType value);

        /// <summary>
        /// Convert C# type to an FFI type
        /// </summary>
        /// <param name="value">The value.</param>
        /// <returns></returns>
        public abstract TFfiType Lower(TCsType value);

        /// <summary>
        /// Read a C# type from a `ByteBuffer`  (<see cref="BigEndianStream"/>)
        /// </summary>
        /// <param name="stream">The stream.</param>
        public abstract TCsType Read(BigEndianStream stream);

        /// <summary>
        /// Calculate bytes to allocate when creating a <see cref="RustBuffer"/>
        ///  This must return at least as many bytes as the write() function will
        ///write.It can return more bytes than needed, for example when writing
        ///    Strings we can't know the exact bytes needed until we use the UTF-8
        /// encoding, so we pessimistically allocate the largest size possible(3
        ///bytes per codepoint).  Allocating extra bytes is not really a big deal
        ///    because the `RustBuffer` is short-lived.
        /// </summary>
        /// <param name="value">The value.</param>
        public abstract int AllocationSize(TCsType value);

        /// <summary>
        /// Writes a C# type to a `ByteBuffer` (<see cref="BigEndianStream"/>)
        /// </summary>
        /// <param name="value">The value.</param>
        /// <param name="stream">The stream.</param>
        public abstract void Write(TCsType value, BigEndianStream stream);

        /// <summary>
        /// Lowers the into <see cref="RustBuffer"/>
        /// This method lowers a value into a `RustBuffer` rather than the normal
        /// FfiType.  It's used by the callback interface code.  Callback interface
        /// returns are always serialized into a `RustBuffer` regardless of their
        /// normal FFI type.
        /// </summary>
        /// <param name="value">The value.</param>
        public RustBuffer LowerIntoRustBuffer(TCsType value)
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

        /// <summary>
        /// Lifts a value from a <see cref="RustBuffer"/>.
        /// This here mostly because of the symmetry with `lowerIntoRustBuffer()`.
        /// It's currently only used by the `FfiConverterRustBuffer` class below.
        /// </summary>
        /// <param name="rbuf">The Rust buffer.</param>
        /// <returns></returns>
        /// <exception cref="InternalException">junk remaining in buffer after lifting, something is very wrong!!</exception>
        protected TCsType LiftFromRustBuffer(RustBuffer rbuf)
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