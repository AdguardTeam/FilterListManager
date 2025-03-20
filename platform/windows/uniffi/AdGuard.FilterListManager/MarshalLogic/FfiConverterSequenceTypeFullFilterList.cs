using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterSequenceTypeFullFilterList : FfiConverterRustBuffer<List<FullFilterList>>
    {
        public static FfiConverterSequenceTypeFullFilterList Instance =
            new FfiConverterSequenceTypeFullFilterList();

        public override List<FullFilterList> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<FullFilterList>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeFullFilterList.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<FullFilterList> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value
                .Select(item => FfiConverterTypeFullFilterList.Instance.AllocationSize(item))
                .Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<FullFilterList> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeFullFilterList.Instance.Write(item, stream));
        }
    }
}