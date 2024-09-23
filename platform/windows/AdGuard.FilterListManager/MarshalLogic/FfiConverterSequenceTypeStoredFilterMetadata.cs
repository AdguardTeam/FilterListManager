using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterSequenceTypeStoredFilterMetadata : FfiConverterRustBuffer<List<StoredFilterMetadata>>
    {
        public static FfiConverterSequenceTypeStoredFilterMetadata Instance = new FfiConverterSequenceTypeStoredFilterMetadata();

        public override List<StoredFilterMetadata> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<StoredFilterMetadata>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeStoredFilterMetadata.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<StoredFilterMetadata> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value.Select(item => FfiConverterTypeStoredFilterMetadata.Instance.AllocationSize(item)).Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<StoredFilterMetadata> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeStoredFilterMetadata.Instance.Write(item, stream));
        }
    }
}
