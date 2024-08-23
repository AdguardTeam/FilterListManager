namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFullFilterList : FfiConverterRustBuffer<FullFilterList>
    {
        public static FfiConverterTypeFullFilterList INSTANCE = new FfiConverterTypeFullFilterList();

        public override FullFilterList Read(BigEndianStream stream)
        {
            return new FullFilterList(
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
                languages: FfiConverterSequenceString.INSTANCE.Read(stream),
                rules: FfiConverterOptionalTypeFilterListRules.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(FullFilterList value)
        {
            return FfiConverterInt64.INSTANCE.AllocationSize(value.id)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.groupId)
                   + FfiConverterInt64.INSTANCE.AllocationSize(value.timeUpdated)
                   + FfiConverterInt64.INSTANCE.AllocationSize(value.lastDownloadTime)
                   + FfiConverterString.INSTANCE.AllocationSize(value.title)
                   + FfiConverterString.INSTANCE.AllocationSize(value.description)
                   + FfiConverterString.INSTANCE.AllocationSize(value.version)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.displayNumber)
                   + FfiConverterString.INSTANCE.AllocationSize(value.downloadUrl)
                   + FfiConverterString.INSTANCE.AllocationSize(value.subscriptionUrl)
                   + FfiConverterSequenceTypeFilterTag.INSTANCE.AllocationSize(value.tags)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.expires)
                   + FfiConverterBoolean.INSTANCE.AllocationSize(value.isTrusted)
                   + FfiConverterBoolean.INSTANCE.AllocationSize(value.isCustom)
                   + FfiConverterBoolean.INSTANCE.AllocationSize(value.isEnabled)
                   + FfiConverterBoolean.INSTANCE.AllocationSize(value.isInstalled)
                   + FfiConverterString.INSTANCE.AllocationSize(value.homepage)
                   + FfiConverterString.INSTANCE.AllocationSize(value.license)
                   + FfiConverterString.INSTANCE.AllocationSize(value.checksum)
                   + FfiConverterSequenceString.INSTANCE.AllocationSize(value.languages)
                   + FfiConverterOptionalTypeFilterListRules.INSTANCE.AllocationSize(value.rules);
        }

        public override void Write(FullFilterList value, BigEndianStream stream)
        {
            FfiConverterInt64.INSTANCE.Write(value.id, stream);
            FfiConverterInt32.INSTANCE.Write(value.groupId, stream);
            FfiConverterInt64.INSTANCE.Write(value.timeUpdated, stream);
            FfiConverterInt64.INSTANCE.Write(value.lastDownloadTime, stream);
            FfiConverterString.INSTANCE.Write(value.title, stream);
            FfiConverterString.INSTANCE.Write(value.description, stream);
            FfiConverterString.INSTANCE.Write(value.version, stream);
            FfiConverterInt32.INSTANCE.Write(value.displayNumber, stream);
            FfiConverterString.INSTANCE.Write(value.subscriptionUrl, stream);
            FfiConverterString.INSTANCE.Write(value.downloadUrl, stream);
            FfiConverterSequenceTypeFilterTag.INSTANCE.Write(value.tags, stream);
            FfiConverterInt32.INSTANCE.Write(value.expires, stream);
            FfiConverterBoolean.INSTANCE.Write(value.isTrusted, stream);
            FfiConverterBoolean.INSTANCE.Write(value.isCustom, stream);
            FfiConverterBoolean.INSTANCE.Write(value.isEnabled, stream);
            FfiConverterBoolean.INSTANCE.Write(value.isInstalled, stream);
            FfiConverterString.INSTANCE.Write(value.homepage, stream);
            FfiConverterString.INSTANCE.Write(value.license, stream);
            FfiConverterString.INSTANCE.Write(value.checksum, stream);
            FfiConverterSequenceString.INSTANCE.Write(value.languages, stream);
            FfiConverterOptionalTypeFilterListRules.INSTANCE.Write(value.rules, stream);
        }
    }
}