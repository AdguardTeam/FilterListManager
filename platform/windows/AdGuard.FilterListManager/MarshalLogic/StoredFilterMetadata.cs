using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Filter metadata info that stored in the filter DB
    /// </summary>
    public class StoredFilterMetadata
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="StoredFilterMetadata"/> class.
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
        /// <param name="isUserTitle">if set to <c>true</c> [is user title].</param>
        /// <param name="isUserDescription">if set to <c>true</c> [is user title].</param>
        public StoredFilterMetadata(int id,
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
            bool isUserTitle,
            bool isUserDescription)
        {
            Id = id;
            GroupId = groupId;
            TimeUpdated = timeUpdated;
            LastDownloadTime = lastDownloadTime;
            Title = title;
            Description = description;
            Version = version;
            DisplayNumber = displayNumber;
            DownloadUrl = downloadUrl;
            SubscriptionUrl = subscriptionUrl;
            Tags = tags;
            Expires = expires;
            IsTrusted = isTrusted;
            IsCustom = isCustom;
            IsEnabled = isEnabled;
            IsInstalled = isInstalled;
            Homepage = homepage;
            License = license;
            Checksum = checksum;
            Languages = languages;
            IsUserTitle = isUserTitle;
            IsUserDescription = isUserDescription;
        }

        /// <summary>
        /// Gets the filter identifier.
        /// </summary>
        public int Id { get; }

        /// <summary>
        /// Gets the group identifier.
        /// </summary>
        public int GroupId { get; }

        /// <summary>
        /// Gets the time updated.
        /// </summary>
        public long TimeUpdated { get; }

        /// <summary>
        /// Gets the last download time.
        /// </summary>
        public long LastDownloadTime { get; }

        /// <summary>
        /// Gets the title.
        /// </summary>
        public string Title { get; }

        /// <summary>
        /// Gets the description.
        /// </summary>
        public string Description { get; }

        /// <summary>
        /// Gets the version.
        /// </summary>
        public string Version { get; }

        /// <summary>
        /// Gets the display number.
        /// </summary>
        public int DisplayNumber { get; }

        /// <summary>
        /// Gets the download URL.
        /// </summary>
        public string DownloadUrl { get; }

        /// <summary>
        /// Gets the subscription URL.
        /// </summary>
        public string SubscriptionUrl { get; }

        /// <summary>
        /// Gets the tags.
        /// </summary>
        public List<FilterTag> Tags { get; }

        /// <summary>
        /// Gets the expiration time in seconds.
        /// </summary>
        public int Expires { get; }

        /// <summary>
        /// Gets a value indicating whether this filter is trusted.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is trusted; otherwise, <c>false</c>.
        /// </value>
        public bool IsTrusted { get; }

        /// <summary>
        /// Gets a value indicating whether this filter is custom.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is custom; otherwise, <c>false</c>.
        /// </value>
        public bool IsCustom { get; }

        /// <summary>
        /// Gets a value indicating whether this filter is enabled.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is enabled; otherwise, <c>false</c>.
        /// </value>
        public bool IsEnabled { get; }

        /// <summary>
        /// Gets a value indicating whether this filter is installed.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is installed; otherwise, <c>false</c>.
        /// </value>
        public bool IsInstalled { get; }

        /// <summary>
        /// Gets the homepage of the filter.
        /// </summary>
        public string Homepage { get; }

        /// <summary>
        /// Gets the license string.
        /// </summary>
        public string License { get; }

        /// <summary>
        /// Gets the checksum.
        /// </summary>
        public string Checksum { get; }

        /// <summary>
        /// Gets the languages of the filter.
        /// </summary>
        public List<string> Languages { get; }

        /// <summary>
        /// Gets a value indicating whether this filter title is user defined.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter title is user defined; otherwise, <c>false</c>.
        /// </value>
        public bool IsUserTitle { get; }

        /// <summary>
        /// Gets a value indicating whether this filter description is user defined.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter description is user defined; otherwise, <c>false</c>.
        /// </value>
        public bool IsUserDescription { get; }
    }
}
