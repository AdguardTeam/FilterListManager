namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterListMetadataWithBody : FfiConverterRustBuffer<FilterListMetadataWithBody>
    {
        public static FfiConverterTypeFilterListMetadataWithBody Instance =
            new FfiConverterTypeFilterListMetadataWithBody();

        public override FilterListMetadataWithBody Read(BigEndianStream stream)
        {
            return new FilterListMetadataWithBody(
                metadata: FfiConverterTypeFilterListMetadata.Instance.Read(stream),
                filterBody: FfiConverterString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FilterListMetadataWithBody value)
        {
            return FfiConverterTypeFilterListMetadata.Instance.AllocationSize(value.Metadata)
                   + FfiConverterString.Instance.AllocationSize(value.FilterBody);
        }

        public override void Write(FilterListMetadataWithBody value, BigEndianStream stream)
        {
            FfiConverterTypeFilterListMetadata.Instance.Write(value.Metadata, stream);
            FfiConverterString.Instance.Write(value.FilterBody, stream);
        }
    }
}