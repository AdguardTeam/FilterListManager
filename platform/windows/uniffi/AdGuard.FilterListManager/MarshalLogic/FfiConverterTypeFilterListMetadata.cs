namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterListMetadata : FfiConverterRustBuffer<FilterListMetadata>
    {
        public static FfiConverterTypeFilterListMetadata Instance =
            new FfiConverterTypeFilterListMetadata();

        public override FilterListMetadata Read(BigEndianStream stream)
        {
            return new FilterListMetadata(
                title: FfiConverterString.Instance.Read(stream),
                description: FfiConverterString.Instance.Read(stream),
                timeUpdated: FfiConverterString.Instance.Read(stream),
                version: FfiConverterString.Instance.Read(stream),
                homepage: FfiConverterString.Instance.Read(stream),
                license: FfiConverterString.Instance.Read(stream),
                checksum: FfiConverterString.Instance.Read(stream),
                url: FfiConverterString.Instance.Read(stream),
                rulesCount: FfiConverterInt32.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FilterListMetadata value)
        {
            return FfiConverterString.Instance.AllocationSize(value.title)
                   + FfiConverterString.Instance.AllocationSize(value.description)
                   + FfiConverterString.Instance.AllocationSize(value.timeUpdated)
                   + FfiConverterString.Instance.AllocationSize(value.version)
                   + FfiConverterString.Instance.AllocationSize(value.homepage)
                   + FfiConverterString.Instance.AllocationSize(value.license)
                   + FfiConverterString.Instance.AllocationSize(value.checksum)
                   + FfiConverterString.Instance.AllocationSize(value.url)
                   + FfiConverterInt32.Instance.AllocationSize(value.rulesCount);
        }

        public override void Write(FilterListMetadata value, BigEndianStream stream)
        {
            FfiConverterString.Instance.Write(value.title, stream);
            FfiConverterString.Instance.Write(value.description, stream);
            FfiConverterString.Instance.Write(value.timeUpdated, stream);
            FfiConverterString.Instance.Write(value.version, stream);
            FfiConverterString.Instance.Write(value.homepage, stream);
            FfiConverterString.Instance.Write(value.license, stream);
            FfiConverterString.Instance.Write(value.checksum, stream);
            FfiConverterString.Instance.Write(value.url, stream);
            FfiConverterInt32.Instance.Write(value.rulesCount, stream);
        }
    }
}