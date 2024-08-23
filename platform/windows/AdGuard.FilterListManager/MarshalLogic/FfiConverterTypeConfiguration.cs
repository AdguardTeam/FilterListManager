namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeConfiguration : FfiConverterRustBuffer<Configuration>
    {
        public static FfiConverterTypeConfiguration INSTANCE = new FfiConverterTypeConfiguration();

        public override Configuration Read(BigEndianStream stream)
        {
            return new Configuration(
                filterListType: FfiConverterTypeFilterListType.INSTANCE.Read(stream),
                workingDirectory: FfiConverterOptionalString.INSTANCE.Read(stream),
                locale: FfiConverterString.INSTANCE.Read(stream),
                defaultFilterListExpiresPeriodSec: FfiConverterInt32.INSTANCE.Read(stream),
                compilerConditionalConstants: FfiConverterOptionalSequenceString.INSTANCE.Read(stream),
                metadataUrl: FfiConverterString.INSTANCE.Read(stream),
                metadataLocalesUrl: FfiConverterString.INSTANCE.Read(stream),
                encryptionKey: FfiConverterOptionalString.INSTANCE.Read(stream),
                requestTimeoutMs: FfiConverterInt32.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(Configuration value)
        {
            return FfiConverterTypeFilterListType.INSTANCE.AllocationSize(value.FilterListType)
                   + FfiConverterOptionalString.INSTANCE.AllocationSize(value.WorkingDirectory)
                   + FfiConverterString.INSTANCE.AllocationSize(value.Locale)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.DefaultFilterListExpiresPeriodSec)
                   + FfiConverterOptionalSequenceString.INSTANCE.AllocationSize(
                       value.CompilerConditionalConstants
                   )
                   + FfiConverterString.INSTANCE.AllocationSize(value.MetadataUrl)
                   + FfiConverterString.INSTANCE.AllocationSize(value.MetadataLocalesUrl)
                   + FfiConverterOptionalString.INSTANCE.AllocationSize(value.EncryptionKey)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.RequestTimeoutMs);
        }

        public override void Write(Configuration value, BigEndianStream stream)
        {
            FfiConverterTypeFilterListType.INSTANCE.Write(value.FilterListType, stream);
            FfiConverterOptionalString.INSTANCE.Write(value.WorkingDirectory, stream);
            FfiConverterString.INSTANCE.Write(value.Locale, stream);
            FfiConverterInt32.INSTANCE.Write(value.DefaultFilterListExpiresPeriodSec, stream);
            FfiConverterOptionalSequenceString.INSTANCE.Write(
                value.CompilerConditionalConstants,
                stream
            );
            FfiConverterString.INSTANCE.Write(value.MetadataUrl, stream);
            FfiConverterString.INSTANCE.Write(value.MetadataLocalesUrl, stream);
            FfiConverterOptionalString.INSTANCE.Write(value.EncryptionKey, stream);
            FfiConverterInt32.INSTANCE.Write(value.RequestTimeoutMs, stream);
        }
    }
}