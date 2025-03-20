namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeConfiguration : FfiConverterRustBuffer<Configuration>
    {
        public static FfiConverterTypeConfiguration Instance = new FfiConverterTypeConfiguration();

        public override Configuration Read(BigEndianStream stream)
        {
            return new Configuration(
                filterListType: FfiConverterTypeFilterListType.Instance.Read(stream),
                workingDirectory: FfiConverterOptionalString.Instance.Read(stream),
                locale: FfiConverterString.Instance.Read(stream),
                defaultFilterListExpiresPeriodSec: FfiConverterInt32.Instance.Read(stream),
                compilerConditionalConstants: FfiConverterOptionalSequenceString.Instance.Read(stream),
                metadataUrl: FfiConverterString.Instance.Read(stream),
                metadataLocalesUrl: FfiConverterString.Instance.Read(stream),
                requestTimeoutMs: FfiConverterInt32.Instance.Read(stream),
                requestProxyMode: FfiConverterTypeRequestProxyMode.Instance.Read(stream),
                autoLiftUpDatabase: FfiConverterBoolean.Instance.Read(stream),
                appName: FfiConverterString.Instance.Read(stream),
                version: FfiConverterString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(Configuration value)
        {
            return FfiConverterTypeFilterListType.Instance.AllocationSize(value.FilterListType)
                   + FfiConverterOptionalString.Instance.AllocationSize(value.WorkingDirectory)
                   + FfiConverterString.Instance.AllocationSize(value.Locale)
                   + FfiConverterInt32.Instance.AllocationSize(value.DefaultFilterListExpiresPeriodSec)
                   + FfiConverterOptionalSequenceString.Instance.AllocationSize(
                       value.CompilerConditionalConstants
                   )
                   + FfiConverterString.Instance.AllocationSize(value.MetadataUrl)
                   + FfiConverterString.Instance.AllocationSize(value.MetadataLocalesUrl)
                   + FfiConverterInt32.Instance.AllocationSize(value.RequestTimeoutMs)
                   + FfiConverterTypeRequestProxyMode.Instance.AllocationSize(value.RequestProxyMode)
                   + FfiConverterBoolean.Instance.AllocationSize(value.AutoLiftUpDatabase)
                   + FfiConverterString.Instance.AllocationSize(value.AppName)
                   + FfiConverterString.Instance.AllocationSize(value.Version);
        }

        public override void Write(Configuration value, BigEndianStream stream)
        {
            FfiConverterTypeFilterListType.Instance.Write(value.FilterListType, stream);
            FfiConverterOptionalString.Instance.Write(value.WorkingDirectory, stream);
            FfiConverterString.Instance.Write(value.Locale, stream);
            FfiConverterInt32.Instance.Write(value.DefaultFilterListExpiresPeriodSec, stream);
            FfiConverterOptionalSequenceString.Instance.Write(
                value.CompilerConditionalConstants,
                stream
            );
            FfiConverterString.Instance.Write(value.MetadataUrl, stream);
            FfiConverterString.Instance.Write(value.MetadataLocalesUrl, stream);
            FfiConverterInt32.Instance.Write(value.RequestTimeoutMs, stream);
            FfiConverterTypeRequestProxyMode.Instance.Write(value.RequestProxyMode, stream);
            FfiConverterBoolean.Instance.Write(value.AutoLiftUpDatabase, stream);
            FfiConverterString.Instance.Write(value.AppName, stream);
            FfiConverterString.Instance.Write(value.Version, stream);
        }
    }
}