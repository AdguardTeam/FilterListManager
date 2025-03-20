using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterSequenceTypeFilterGroup : FfiConverterRustBuffer<List<FilterGroup>>
    {
        public static FfiConverterSequenceTypeFilterGroup Instance =
            new FfiConverterSequenceTypeFilterGroup();

        public override List<FilterGroup> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<FilterGroup>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeFilterGroup.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<FilterGroup> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value
                .Select(item => FfiConverterTypeFilterGroup.Instance.AllocationSize(item))
                .Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<FilterGroup> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeFilterGroup.Instance.Write(item, stream));
        }
    }
}