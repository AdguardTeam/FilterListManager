namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFullFilterList : FfiConverterRustBuffer<FullFilterList>
    {
        public static FfiConverterTypeFullFilterList Instance = new FfiConverterTypeFullFilterList();

        public override FullFilterList Read(BigEndianStream stream)
        {
            return new FullFilterList(
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
                languages: FfiConverterSequenceString.Instance.Read(stream),
                rules: FfiConverterOptionalTypeFilterListRules.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FullFilterList value)
        {
            return FfiConverterInt64.Instance.AllocationSize(value.id)
                   + FfiConverterInt32.Instance.AllocationSize(value.groupId)
                   + FfiConverterInt64.Instance.AllocationSize(value.timeUpdated)
                   + FfiConverterInt64.Instance.AllocationSize(value.lastDownloadTime)
                   + FfiConverterString.Instance.AllocationSize(value.title)
                   + FfiConverterString.Instance.AllocationSize(value.description)
                   + FfiConverterString.Instance.AllocationSize(value.version)
                   + FfiConverterInt32.Instance.AllocationSize(value.displayNumber)
                   + FfiConverterString.Instance.AllocationSize(value.downloadUrl)
                   + FfiConverterString.Instance.AllocationSize(value.subscriptionUrl)
                   + FfiConverterSequenceTypeFilterTag.Instance.AllocationSize(value.tags)
                   + FfiConverterInt32.Instance.AllocationSize(value.expires)
                   + FfiConverterBoolean.Instance.AllocationSize(value.isTrusted)
                   + FfiConverterBoolean.Instance.AllocationSize(value.isCustom)
                   + FfiConverterBoolean.Instance.AllocationSize(value.isEnabled)
                   + FfiConverterBoolean.Instance.AllocationSize(value.isInstalled)
                   + FfiConverterString.Instance.AllocationSize(value.homepage)
                   + FfiConverterString.Instance.AllocationSize(value.license)
                   + FfiConverterString.Instance.AllocationSize(value.checksum)
                   + FfiConverterSequenceString.Instance.AllocationSize(value.languages)
                   + FfiConverterOptionalTypeFilterListRules.Instance.AllocationSize(value.rules);
        }

        public override void Write(FullFilterList value, BigEndianStream stream)
        {
            FfiConverterInt64.Instance.Write(value.id, stream);
            FfiConverterInt32.Instance.Write(value.groupId, stream);
            FfiConverterInt64.Instance.Write(value.timeUpdated, stream);
            FfiConverterInt64.Instance.Write(value.lastDownloadTime, stream);
            FfiConverterString.Instance.Write(value.title, stream);
            FfiConverterString.Instance.Write(value.description, stream);
            FfiConverterString.Instance.Write(value.version, stream);
            FfiConverterInt32.Instance.Write(value.displayNumber, stream);
            FfiConverterString.Instance.Write(value.subscriptionUrl, stream);
            FfiConverterString.Instance.Write(value.downloadUrl, stream);
            FfiConverterSequenceTypeFilterTag.Instance.Write(value.tags, stream);
            FfiConverterInt32.Instance.Write(value.expires, stream);
            FfiConverterBoolean.Instance.Write(value.isTrusted, stream);
            FfiConverterBoolean.Instance.Write(value.isCustom, stream);
            FfiConverterBoolean.Instance.Write(value.isEnabled, stream);
            FfiConverterBoolean.Instance.Write(value.isInstalled, stream);
            FfiConverterString.Instance.Write(value.homepage, stream);
            FfiConverterString.Instance.Write(value.license, stream);
            FfiConverterString.Instance.Write(value.checksum, stream);
            FfiConverterSequenceString.Instance.Write(value.languages, stream);
            FfiConverterOptionalTypeFilterListRules.Instance.Write(value.rules, stream);
        }
    }
}