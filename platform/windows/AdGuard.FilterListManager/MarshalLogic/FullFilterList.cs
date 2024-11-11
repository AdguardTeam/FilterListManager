using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Fill info about the filter from the list
    /// </summary>
    public class FullFilterList
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FullFilterList"/> class.
        /// </summary>
        /// <param name="id">The identifier.</param>
        /// <param name="groupId">The group identifier.</param>
        /// <param name="timeUpdated">The time updated.</param>
        /// <param name="lastDownloadTime">The last download time.</param>
        /// <param name="title">The title.</param>
        /// <param name="description">The description.</param>
        /// <param name="version">The version.</param>
        /// <param name="displayNumber">The display number.</param>
        /// <param name="downloadUrl">The download URL.</param>
        /// <param name="subscriptionUrl">The subscription URL.</param>
        /// <param name="tags">The tags.</param>
        /// <param name="expires">The expires.</param>
        /// <param name="isTrusted">if set to <c>true</c> [is trusted].</param>
        /// <param name="isCustom">if set to <c>true</c> [is custom].</param>
        /// <param name="isEnabled">if set to <c>true</c> [is enabled].</param>
        /// <param name="isInstalled">if set to <c>true</c> [is installed].</param>
        /// <param name="homepage">The homepage.</param>
        /// <param name="license">The license.</param>
        /// <param name="checksum">The checksum.</param>
        /// <param name="languages">The languages.</param>
        /// <param name="rules">The rules.</param>
        public FullFilterList(long id,
            int groupId,
            long timeUpdated,
            long lastDownloadTime,
            string title,
            string description,
            string version,
            int displayNumber,
            string downloadUrl,
            string subscriptionUrl,
            List<FilterTag> tags,
            int expires,
            bool isTrusted,
            bool isCustom,
            bool isEnabled,
            bool isInstalled,
            string homepage,
            string license,
            string checksum,
            List<string> languages,
            FilterListRules rules)
        {
            this.id = id;
            this.groupId = groupId;
            this.timeUpdated = timeUpdated;
            this.lastDownloadTime = lastDownloadTime;
            this.title = title;
            this.description = description;
            this.version = version;
            this.displayNumber = displayNumber;
            this.downloadUrl = downloadUrl;
            this.subscriptionUrl = subscriptionUrl;
            this.tags = tags;
            this.expires = expires;
            this.isTrusted = isTrusted;
            this.isCustom = isCustom;
            this.isEnabled = isEnabled;
            this.isInstalled = isInstalled;
            this.homepage = homepage;
            this.license = license;
            this.checksum = checksum;
            this.languages = languages;
            this.rules = rules;
        }

        /// <summary>
        /// Gets or sets the filter identifier.
        /// </summary>
        public long id { get; set; }

        /// <summary>
        /// Gets or sets the group identifier.
        /// </summary>
        public int groupId { get; set; }

        /// <summary>
        /// Gets or sets the time updated.
        /// </summary>
        public long timeUpdated { get; set; }

        /// <summary>
        /// Gets or sets the last download time.
        /// </summary>
        public long lastDownloadTime { get; set; }

        /// <summary>
        /// Gets or sets the title.
        /// </summary>
        public string title { get; set; }

        /// <summary>
        /// Gets or sets the description.
        /// </summary>
        public string description { get; set; }

        /// <summary>
        /// Gets or sets the version.
        /// </summary>
        public string version { get; set; }

        /// <summary>
        /// Gets or sets the display number.
        /// </summary>
        public int displayNumber { get; set; }

        /// <summary>
        /// Gets or sets the download URL.
        /// </summary>
        public string downloadUrl { get; set; }

        /// <summary>
        /// Gets or sets the subscription URL.
        /// </summary>
        public string subscriptionUrl { get; set; }

        /// <summary>
        /// Gets or sets the tags.
        /// </summary>
        public List<FilterTag> tags { get; set; }

        /// <summary>
        /// Gets or sets when this filter the expires.
        /// </summary>
        public int expires { get; set; }

        /// <summary>
        /// Gets or sets a value indicating whether this filter is trusted.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is trusted; otherwise, <c>false</c>.
        /// </value>
        public bool isTrusted { get; set; }

        /// <summary>
        /// Gets or sets a value indicating whether this filter is custom.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is custom; otherwise, <c>false</c>.
        /// </value>
        public bool isCustom { get; set; }

        /// <summary>
        /// Gets or sets a value indicating whether this filter is enabled.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is enabled; otherwise, <c>false</c>.
        /// </value>
        public bool isEnabled { get; set; }

        /// <summary>
        /// Gets or sets a value indicating whether this filter is installed.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is installed; otherwise, <c>false</c>.
        /// </value>
        public bool isInstalled { get; set; }

        /// <summary>
        /// Gets or sets the homepage of the filter.
        /// </summary>
        public string homepage { get; set; }

        /// <summary>
        /// Gets or sets the license string.
        /// </summary>
        public string license { get; set; }

        /// <summary>
        /// Gets or sets the checksum.
        /// </summary>
        public string checksum { get; set; }

        /// <summary>
        /// Gets or sets the languages of the filter.
        /// </summary>
        public List<string> languages { get; set; }
        
        /// <summary>
        /// Gets or sets the rules entity of th filter.
        /// </summary>
        public FilterListRules rules { get; set; }
    }
}