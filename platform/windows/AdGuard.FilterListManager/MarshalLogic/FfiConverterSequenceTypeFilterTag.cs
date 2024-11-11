using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterSequenceTypeFilterTag : FfiConverterRustBuffer<List<FilterTag>>
    {
        public static FfiConverterSequenceTypeFilterTag Instance =
            new FfiConverterSequenceTypeFilterTag();

        public override List<FilterTag> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<FilterTag>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeFilterTag.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<FilterTag> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value
                .Select(item => FfiConverterTypeFilterTag.Instance.AllocationSize(item))
                .Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<FilterTag> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeFilterTag.Instance.Write(item, stream));
        }
    }
}