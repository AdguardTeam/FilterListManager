namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeStoredFilterMetadata : FfiConverterRustBuffer<StoredFilterMetadata>
    {
        public static FfiConverterTypeStoredFilterMetadata Instance = new FfiConverterTypeStoredFilterMetadata();

        public override StoredFilterMetadata Read(BigEndianStream stream)
        {
            return new StoredFilterMetadata(
                id: FfiConverterInt64.INSTANCE.Read(stream),
                groupId: FfiConverterInt32.INSTANCE.Read(stream),
                timeUpdated: FfiConverterInt64.INSTANCE.Read(stream),
                lastDownloadTime: FfiConverterInt64.INSTANCE.Read(stream),
                title: FfiConverterString.INSTANCE.Read(stream),
                description: FfiConverterString.INSTANCE.Read(stream),
                version: FfiConverterString.INSTANCE.Read(stream),
                displayNumber: FfiConverterInt32.INSTANCE.Read(stream),
                downloadUrl: FfiConverterString.INSTANCE.Read(stream),
                subscriptionUrl: FfiConverterString.INSTANCE.Read(stream),
                tags: FfiConverterSequenceTypeFilterTag.INSTANCE.Read(stream),
                expires: FfiConverterInt32.INSTANCE.Read(stream),
                isTrusted: FfiConverterBoolean.INSTANCE.Read(stream),
                isCustom: FfiConverterBoolean.INSTANCE.Read(stream),
                isEnabled: FfiConverterBoolean.INSTANCE.Read(stream),
                isInstalled: FfiConverterBoolean.INSTANCE.Read(stream),
                homepage: FfiConverterString.INSTANCE.Read(stream),
                license: FfiConverterString.INSTANCE.Read(stream),
                checksum: FfiConverterString.INSTANCE.Read(stream),
                languages: FfiConverterSequenceString.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(StoredFilterMetadata value)
        {
            return
                FfiConverterInt64.INSTANCE.AllocationSize(value.Id) +
                FfiConverterInt32.INSTANCE.AllocationSize(value.GroupId) +
                FfiConverterInt64.INSTANCE.AllocationSize(value.TimeUpdated) +
                FfiConverterInt64.INSTANCE.AllocationSize(value.LastDownloadTime) +
                FfiConverterString.INSTANCE.AllocationSize(value.Title) +
                FfiConverterString.INSTANCE.AllocationSize(value.Description) +
                FfiConverterString.INSTANCE.AllocationSize(value.Version) +
                FfiConverterInt32.INSTANCE.AllocationSize(value.DisplayNumber) +
                FfiConverterString.INSTANCE.AllocationSize(value.DownloadUrl) +
                FfiConverterString.INSTANCE.AllocationSize(value.SubscriptionUrl) +
                FfiConverterSequenceTypeFilterTag.INSTANCE.AllocationSize(value.Tags) +
                FfiConverterInt32.INSTANCE.AllocationSize(value.Expires) +
                FfiConverterBoolean.INSTANCE.AllocationSize(value.IsTrusted) +
                FfiConverterBoolean.INSTANCE.AllocationSize(value.IsCustom) +
                FfiConverterBoolean.INSTANCE.AllocationSize(value.IsEnabled) +
                FfiConverterBoolean.INSTANCE.AllocationSize(value.IsInstalled) +
                FfiConverterString.INSTANCE.AllocationSize(value.Homepage) +
                FfiConverterString.INSTANCE.AllocationSize(value.License) +
                FfiConverterString.INSTANCE.AllocationSize(value.Checksum) +
                FfiConverterSequenceString.INSTANCE.AllocationSize(value.Languages);
        }

        public override void Write(StoredFilterMetadata value, BigEndianStream stream)
        {
            FfiConverterInt64.INSTANCE.Write(value.Id, stream);
            FfiConverterInt32.INSTANCE.Write(value.GroupId, stream);
            FfiConverterInt64.INSTANCE.Write(value.TimeUpdated, stream);
            FfiConverterInt64.INSTANCE.Write(value.LastDownloadTime, stream);
            FfiConverterString.INSTANCE.Write(value.Title, stream);
            FfiConverterString.INSTANCE.Write(value.Description, stream);
            FfiConverterString.INSTANCE.Write(value.Version, stream);
            FfiConverterInt32.INSTANCE.Write(value.DisplayNumber, stream);
            FfiConverterString.INSTANCE.Write(value.DownloadUrl, stream);
            FfiConverterString.INSTANCE.Write(value.SubscriptionUrl, stream);
            FfiConverterSequenceTypeFilterTag.INSTANCE.Write(value.Tags, stream);
            FfiConverterInt32.INSTANCE.Write(value.Expires, stream);
            FfiConverterBoolean.INSTANCE.Write(value.IsTrusted, stream);
            FfiConverterBoolean.INSTANCE.Write(value.IsCustom, stream);
            FfiConverterBoolean.INSTANCE.Write(value.IsEnabled, stream);
            FfiConverterBoolean.INSTANCE.Write(value.IsInstalled, stream);
            FfiConverterString.INSTANCE.Write(value.Homepage, stream);
            FfiConverterString.INSTANCE.Write(value.License, stream);
            FfiConverterString.INSTANCE.Write(value.Checksum, stream);
            FfiConverterSequenceString.INSTANCE.Write(value.Languages, stream);
        }
    }
}
