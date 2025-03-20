namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterOptionalTypeStoredFilterMetadata : FfiConverterRustBuffer<StoredFilterMetadata>
    {
        public static FfiConverterOptionalTypeStoredFilterMetadata Instance = new FfiConverterOptionalTypeStoredFilterMetadata();

        public override StoredFilterMetadata Read(BigEndianStream stream)
        {
            if (stream.ReadByte() == 0)
            {
                return null;
            }
            return FfiConverterTypeStoredFilterMetadata.Instance.Read(stream);
        }

        public override int AllocationSize(StoredFilterMetadata value)
        {
            if (value == null)
            {
                return 1;
            }
            else
            {
                return 1 + FfiConverterTypeStoredFilterMetadata.Instance.AllocationSize((StoredFilterMetadata)value);
            }
        }

        public override void Write(StoredFilterMetadata value, BigEndianStream stream)
        {
            if (value == null)
            {
                stream.WriteByte(0);
            }
            else
            {
                stream.WriteByte(1);
                FfiConverterTypeStoredFilterMetadata.Instance.Write((StoredFilterMetadata)value, stream);
            }
        }
    }
}
