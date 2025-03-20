using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterSequenceInt32 : FfiConverterRustBuffer<List<int>>
    {
        public static FfiConverterSequenceInt32 Instance = new FfiConverterSequenceInt32();

        public override List<int> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<int>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterInt32.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<int> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value.Select(item => FfiConverterInt32.Instance.AllocationSize(item)).Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<int> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterInt32.Instance.Write(item, stream));
        }
    }
}
