namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFilterListMetadata : FfiConverterRustBuffer<FilterListMetadata>
    {
        public static FfiConverterTypeFilterListMetadata INSTANCE =
            new FfiConverterTypeFilterListMetadata();

        public override FilterListMetadata Read(BigEndianStream stream)
        {
            return new FilterListMetadata(
                title: FfiConverterString.INSTANCE.Read(stream),
                description: FfiConverterString.INSTANCE.Read(stream),
                timeUpdated: FfiConverterString.INSTANCE.Read(stream),
                version: FfiConverterString.INSTANCE.Read(stream),
                homepage: FfiConverterString.INSTANCE.Read(stream),
                license: FfiConverterString.INSTANCE.Read(stream),
                checksum: FfiConverterString.INSTANCE.Read(stream),
                url: FfiConverterString.INSTANCE.Read(stream),
                rulesCount: FfiConverterInt32.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(FilterListMetadata value)
        {
            return FfiConverterString.INSTANCE.AllocationSize(value.title)
                   + FfiConverterString.INSTANCE.AllocationSize(value.description)
                   + FfiConverterString.INSTANCE.AllocationSize(value.timeUpdated)
                   + FfiConverterString.INSTANCE.AllocationSize(value.version)
                   + FfiConverterString.INSTANCE.AllocationSize(value.homepage)
                   + FfiConverterString.INSTANCE.AllocationSize(value.license)
                   + FfiConverterString.INSTANCE.AllocationSize(value.checksum)
                   + FfiConverterString.INSTANCE.AllocationSize(value.url)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.rulesCount);
        }

        public override void Write(FilterListMetadata value, BigEndianStream stream)
        {
            FfiConverterString.INSTANCE.Write(value.title, stream);
            FfiConverterString.INSTANCE.Write(value.description, stream);
            FfiConverterString.INSTANCE.Write(value.timeUpdated, stream);
            FfiConverterString.INSTANCE.Write(value.version, stream);
            FfiConverterString.INSTANCE.Write(value.homepage, stream);
            FfiConverterString.INSTANCE.Write(value.license, stream);
            FfiConverterString.INSTANCE.Write(value.checksum, stream);
            FfiConverterString.INSTANCE.Write(value.url, stream);
            FfiConverterInt32.INSTANCE.Write(value.rulesCount, stream);
        }
    }
}