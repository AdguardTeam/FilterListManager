using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Configuration class for Filter Manager <see cref="IFilterListManager"/>
    /// </summary>
    public class Configuration
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="Configuration"/> class.
        /// </summary>
        /// <param name="filterListType">Type of the filter list.</param>
        /// <param name="workingDirectory">The working directory.</param>
        /// <param name="locale">The locale.</param>
        /// <param name="defaultFilterListExpiresPeriodSec">The default filter list expires period sec.</param>
        /// <param name="compilerConditionalConstants">The compiler conditional constants.</param>
        /// <param name="metadataUrl">The metadata URL.</param>
        /// <param name="metadataLocalesUrl">The metadata locales URL.</param>
        /// <param name="requestTimeoutMs">The request timeout ms.</param>
        /// <param name="autoLiftUpDatabase">if set to <c>true</c> the database will be lifted (up version) automatically.</param>
        public Configuration(FilterListType filterListType,
            string workingDirectory,
            string locale,
            int defaultFilterListExpiresPeriodSec,
            List<string> compilerConditionalConstants,
            string metadataUrl,
            string metadataLocalesUrl,
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
            RequestTimeoutMs = requestTimeoutMs;
            AutoLiftUpDatabase = autoLiftUpDatabase;
        }

        /// <summary>
        /// Gets or sets the type of the filter list.
        /// </summary>
        public FilterListType FilterListType { get; set; }

        /// <summary>
        /// Gets or sets the working directory.
        /// </summary>
        public string WorkingDirectory { get; set; }

        /// <summary>
        /// Gets or sets the locale.
        /// </summary>
        public string Locale { get; set; }

        /// <summary>
        /// Gets or sets the default filter list expires period in seconds.
        /// </summary>
        public int DefaultFilterListExpiresPeriodSec { get; set; }

        /// <summary>
        /// Gets or sets the filter compiler conditional constants.
        /// </summary>
        public List<string> CompilerConditionalConstants { get; set; }

        /// <summary>
        /// Gets or sets the metadata URL where should we download the filter metadata.
        /// </summary>
        public string MetadataUrl { get; set; }

        /// <summary>
        /// Gets or sets the metadata URL for locales.
        /// </summary>
        public string MetadataLocalesUrl { get; set; }

        /// <summary>
        /// Gets or sets the request timeout in milliseconds.
        /// </summary>
        public int RequestTimeoutMs { get; set; }

        /// <summary>
        /// Gets or sets a value indicating whether the database will be lifted (up version) automatically.
        /// </summary>
        /// <value>
        ///   If set to <c>true</c> the database will be lifted (up version) automatically; otherwise, <c>false</c>.
        /// </value>
        public bool AutoLiftUpDatabase { get; set; }
    }

}
