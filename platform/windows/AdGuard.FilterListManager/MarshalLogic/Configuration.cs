using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class Configuration
    {
        public Configuration(FilterListType filterListType,
            string workingDirectory,
            string locale,
            int defaultFilterListExpiresPeriodSec,
            List<string> compilerConditionalConstants,
            string metadataUrl,
            string metadataLocalesUrl,
            string encryptionKey,
            int requestTimeoutMs,
            bool autoLiftUpDatabase)
        {
            FilterListType = filterListType;
            WorkingDirectory = workingDirectory;
            Locale = locale;
            DefaultFilterListExpiresPeriodSec = defaultFilterListExpiresPeriodSec;
            CompilerConditionalConstants = compilerConditionalConstants;
            MetadataUrl = metadataUrl;
            MetadataLocalesUrl = metadataLocalesUrl;
            EncryptionKey = encryptionKey;
            RequestTimeoutMs = requestTimeoutMs;
            AutoLiftUpDatabase = autoLiftUpDatabase;
        }

        public FilterListType FilterListType { get; set; }

        public string WorkingDirectory { get; set; }

        public string Locale { get; set; }

        public int DefaultFilterListExpiresPeriodSec { get; set; }

        public List<string> CompilerConditionalConstants { get; set; }

        public string MetadataUrl { get; set; }

        public string MetadataLocalesUrl { get; set; }

        public string EncryptionKey { get; set; }

        public int RequestTimeoutMs { get; set; }

        public bool AutoLiftUpDatabase { get; set; }
    }

}
