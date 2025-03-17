namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeFullFilterList : FfiConverterRustBuffer<FullFilterList>
    {
        public static FfiConverterTypeFullFilterList Instance = new FfiConverterTypeFullFilterList();

        public override FullFilterList Read(BigEndianStream stream)
        {
            return new FullFilterList(
                id: FfiConverterInt32.Instance.Read(stream),
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
                rules: FfiConverterOptionalTypeFilterListRules.Instance.Read(stream),
                isUserTitle: FfiConverterBoolean.Instance.Read(stream),
                isUserDescription: FfiConverterBoolean.Instance.Read(stream)
            );
        }

        public override int AllocationSize(FullFilterList value)
        {
            return FfiConverterInt32.Instance.AllocationSize(value.Id)
                   + FfiConverterInt32.Instance.AllocationSize(value.GroupId)
                   + FfiConverterInt64.Instance.AllocationSize(value.TimeUpdated)
                   + FfiConverterInt64.Instance.AllocationSize(value.LastDownloadTime)
                   + FfiConverterString.Instance.AllocationSize(value.Title)
                   + FfiConverterString.Instance.AllocationSize(value.Description)
                   + FfiConverterString.Instance.AllocationSize(value.Version)
                   + FfiConverterInt32.Instance.AllocationSize(value.DisplayNumber)
                   + FfiConverterString.Instance.AllocationSize(value.DownloadUrl)
                   + FfiConverterString.Instance.AllocationSize(value.SubscriptionUrl)
                   + FfiConverterSequenceTypeFilterTag.Instance.AllocationSize(value.Tags)
                   + FfiConverterInt32.Instance.AllocationSize(value.Expires)
                   + FfiConverterBoolean.Instance.AllocationSize(value.IsTrusted)
                   + FfiConverterBoolean.Instance.AllocationSize(value.IsCustom)
                   + FfiConverterBoolean.Instance.AllocationSize(value.IsEnabled)
                   + FfiConverterBoolean.Instance.AllocationSize(value.IsInstalled)
                   + FfiConverterString.Instance.AllocationSize(value.Homepage)
                   + FfiConverterString.Instance.AllocationSize(value.License)
                   + FfiConverterString.Instance.AllocationSize(value.Checksum)
                   + FfiConverterSequenceString.Instance.AllocationSize(value.Languages)
                   + FfiConverterOptionalTypeFilterListRules.Instance.AllocationSize(value.Rules)
                   + FfiConverterBoolean.Instance.AllocationSize(value.IsUserTitle)
                   + FfiConverterBoolean.Instance.AllocationSize(value.IsUserDescription);
        }

        public override void Write(FullFilterList value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.Id, stream);
            FfiConverterInt32.Instance.Write(value.GroupId, stream);
            FfiConverterInt64.Instance.Write(value.TimeUpdated, stream);
            FfiConverterInt64.Instance.Write(value.LastDownloadTime, stream);
            FfiConverterString.Instance.Write(value.Title, stream);
            FfiConverterString.Instance.Write(value.Description, stream);
            FfiConverterString.Instance.Write(value.Version, stream);
            FfiConverterInt32.Instance.Write(value.DisplayNumber, stream);
            FfiConverterString.Instance.Write(value.SubscriptionUrl, stream);
            FfiConverterString.Instance.Write(value.DownloadUrl, stream);
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
            FfiConverterOptionalTypeFilterListRules.Instance.Write(value.Rules, stream);
            FfiConverterBoolean.Instance.Write(value.IsUserTitle, stream);
            FfiConverterBoolean.Instance.Write(value.IsUserDescription, stream);
        }
    }
}