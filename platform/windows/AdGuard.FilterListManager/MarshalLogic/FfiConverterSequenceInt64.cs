using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterSequenceInt64 : FfiConverterRustBuffer<List<long>>
    {
        public static FfiConverterSequenceInt64 Instance = new FfiConverterSequenceInt64();

        public override List<long> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<long>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterInt32.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<long> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value
                .Select(item => FfiConverterInt64.Instance.AllocationSize(item))
                .Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<long> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterInt64.Instance.Write(item, stream));
        }
    }
}