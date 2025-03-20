using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterOptionalSequenceString : FfiConverterRustBuffer<List<string>>
    {
        public static FfiConverterOptionalSequenceString Instance =
            new FfiConverterOptionalSequenceString();

        public override List<string> Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterSequenceString.Instance.Read(stream);
        }

        public override int AllocationSize(List<string> value)
        {
            if (value == null)
            {
                return 1;
            }

            return 1 + FfiConverterSequenceString.Instance.AllocationSize(value);
        }

        public override void Write(List<string> value, BigEndianStream stream)
        {
            if (value == null)
            {
                stream.WriteByte(0);
            }
            else
            {
                stream.WriteByte(1);
                FfiConverterSequenceString.Instance.Write(value, stream);
            }
        }
    }
}