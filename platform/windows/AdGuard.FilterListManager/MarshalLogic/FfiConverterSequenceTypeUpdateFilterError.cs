using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterSequenceTypeUpdateFilterError : FfiConverterRustBuffer<List<UpdateFilterError>>
    {
        public static FfiConverterSequenceTypeUpdateFilterError INSTANCE =
            new FfiConverterSequenceTypeUpdateFilterError();

        public override List<UpdateFilterError> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<UpdateFilterError>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeUpdateFilterError.INSTANCE.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<UpdateFilterError> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value
                .Select(item => FfiConverterTypeUpdateFilterError.INSTANCE.AllocationSize(item))
                .Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<UpdateFilterError> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeUpdateFilterError.INSTANCE.Write(item, stream));
        }
    }
}