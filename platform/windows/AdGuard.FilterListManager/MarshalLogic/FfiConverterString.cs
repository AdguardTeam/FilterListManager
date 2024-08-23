namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterString : FfiConverter<string, RustBuffer>
    {
        public static FfiConverterString INSTANCE = new FfiConverterString();

        // Note: we don't inherit from FfiConverterRustBuffer, because we use a
        // special encoding when lowering/lifting.  We can use `RustBuffer.len` to
        // store our length and avoid writing it out to the buffer.
        public override string Lift(RustBuffer value)
        {
            try
            {
                var bytes = value.AsStream().ReadBytes(value.len);
                return System.Text.Encoding.UTF8.GetString(bytes);
            }
            finally
            {
                RustBuffer.Free(value);
            }
        }

        public override string Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var bytes = stream.ReadBytes(length);
            return System.Text.Encoding.UTF8.GetString(bytes);
        }

        public override RustBuffer Lower(string value)
        {
            var bytes = System.Text.Encoding.UTF8.GetBytes(value);
            var rbuf = RustBuffer.Alloc(bytes.Length);
            rbuf.AsWriteableStream().WriteBytes(bytes);
            return rbuf;
        }

        // TODO(CS)
        // We aren't sure exactly how many bytes our string will be once it's UTF-8
        // encoded.  Allocate 3 bytes per unicode codepoint which will always be
        // enough.
        public override int AllocationSize(string value)
        {
            const int sizeForLength = 4;
            var sizeForString = value.Length * 3;
            return sizeForLength + sizeForString;
        }

        public override void Write(string value, BigEndianStream stream)
        {
            var bytes = System.Text.Encoding.UTF8.GetBytes(value);
            stream.WriteInt(bytes.Length);
            stream.WriteBytes(bytes);
        }
    }
}