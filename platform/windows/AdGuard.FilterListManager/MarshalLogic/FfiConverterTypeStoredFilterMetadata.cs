namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeStoredFilterMetadata : FfiConverterRustBuffer<StoredFilterMetadata>
    {
        public static FfiConverterTypeStoredFilterMetadata Instance = new FfiConverterTypeStoredFilterMetadata();

        public override StoredFilterMetadata Read(BigEndianStream stream)
        {
            return new StoredFilterMetadata(
                id: FfiConverterInt64.Instance.Read(stream),
                groupId: FfiConverterInt32.Instance.Read(stream),
                timeUpdated: FfiConverterInt64.Instance.Read(stream),
                lastDownloadTime: FfiConverterInt64.Instance.Read(stream),
                title: FfiConverterString.Instance.Read(stream),
                description: FfiConverterString.Instance.Read(stream),
                version: FfiConverterString.Instance.Read(stream),
                displayNumber: FfiConverterInt32.Instance.Read(stream),
                downloadUrl: FfiConverterString.Instance.Read(stream),
                subscriptionUrl: FfiConverterString.Instance.Read(stream),
                tags: FfiConverterSequenceTypeFilterTag.Instance.Read(stream),
                expires: FfiConverterInt32.Instance.Read(stream),
                isTrusted: FfiConverterBoolean.Instance.Read(stream),
                isCustom: FfiConverterBoolean.Instance.Read(stream),
                isEnabled: FfiConverterBoolean.Instance.Read(stream),
                isInstalled: FfiConverterBoolean.Instance.Read(stream),
                homepage: FfiConverterString.Instance.Read(stream),
                license: FfiConverterString.Instance.Read(stream),
                checksum: FfiConverterString.Instance.Read(stream),
                languages: FfiConverterSequenceString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(StoredFilterMetadata value)
        {
            return
                FfiConverterInt64.Instance.AllocationSize(value.Id) +
                FfiConverterInt32.Instance.AllocationSize(value.GroupId) +
                FfiConverterInt64.Instance.AllocationSize(value.TimeUpdated) +
                FfiConverterInt64.Instance.AllocationSize(value.LastDownloadTime) +
                FfiConverterString.Instance.AllocationSize(value.Title) +
                FfiConverterString.Instance.AllocationSize(value.Description) +
                FfiConverterString.Instance.AllocationSize(value.Version) +
                FfiConverterInt32.Instance.AllocationSize(value.DisplayNumber) +
                FfiConverterString.Instance.AllocationSize(value.DownloadUrl) +
                FfiConverterString.Instance.AllocationSize(value.SubscriptionUrl) +
                FfiConverterSequenceTypeFilterTag.Instance.AllocationSize(value.Tags) +
                FfiConverterInt32.Instance.AllocationSize(value.Expires) +
                FfiConverterBoolean.Instance.AllocationSize(value.IsTrusted) +
                FfiConverterBoolean.Instance.AllocationSize(value.IsCustom) +
                FfiConverterBoolean.Instance.AllocationSize(value.IsEnabled) +
                FfiConverterBoolean.Instance.AllocationSize(value.IsInstalled) +
                FfiConverterString.Instance.AllocationSize(value.Homepage) +
                FfiConverterString.Instance.AllocationSize(value.License) +
                FfiConverterString.Instance.AllocationSize(value.Checksum) +
                FfiConverterSequenceString.Instance.AllocationSize(value.Languages);
        }

        public override void Write(StoredFilterMetadata value, BigEndianStream stream)
        {
            FfiConverterInt64.Instance.Write(value.Id, stream);
            FfiConverterInt32.Instance.Write(value.GroupId, stream);
            FfiConverterInt64.Instance.Write(value.TimeUpdated, stream);
            FfiConverterInt64.Instance.Write(value.LastDownloadTime, stream);
            FfiConverterString.Instance.Write(value.Title, stream);
            FfiConverterString.Instance.Write(value.Description, stream);
            FfiConverterString.Instance.Write(value.Version, stream);
            FfiConverterInt32.Instance.Write(value.DisplayNumber, stream);
            FfiConverterString.Instance.Write(value.DownloadUrl, stream);
            FfiConverterString.Instance.Write(value.SubscriptionUrl, stream);
            FfiConverterSequenceTypeFilterTag.Instance.Write(value.Tags, stream);
            FfiConverterInt32.Instance.Write(value.Expires, stream);
            FfiConverterBoolean.Instance.Write(value.IsTrusted, stream);
            FfiConverterBoolean.Instance.Write(value.IsCustom, stream);
            FfiConverterBoolean.Instance.Write(value.IsEnabled, stream);
            FfiConverterBoolean.Instance.Write(value.IsInstalled, stream);
            FfiConverterString.Instance.Write(value.Homepage, stream);
            FfiConverterString.Instance.Write(value.License, stream);
            FfiConverterString.Instance.Write(value.Checksum, stream);
            FfiConverterSequenceString.Instance.Write(value.Languages, stream);
        }
    }
}
