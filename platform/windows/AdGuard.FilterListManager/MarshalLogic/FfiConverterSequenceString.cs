using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterSequenceString : FfiConverterRustBuffer<List<string>>
    {
        public static FfiConverterSequenceString Instance = new FfiConverterSequenceString();

        public override List<string> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<string>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterString.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<string> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value
                .Select(item => FfiConverterString.Instance.AllocationSize(item))
                .Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<string> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterString.Instance.Write(item, stream));
        }
    }
}