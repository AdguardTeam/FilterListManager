using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class StoredFilterMetadata
    {
        public StoredFilterMetadata(long id,
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
            List<string> languages)
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
        }

        public long Id { get; set; }
        public int GroupId { get; set; }
        public long TimeUpdated { get; set; }
        public long LastDownloadTime { get; set; }
        public string Title { get; set; }
        public string Description { get; set; }
        public string Version { get; set; }
        public int DisplayNumber { get; set; }
        public string DownloadUrl { get; set; }
        public string SubscriptionUrl { get; set; }
        public List<FilterTag> Tags { get; set; }
        public int Expires { get; set; }
        public bool IsTrusted { get; set; }
        public bool IsCustom { get; set; }
        public bool IsEnabled { get; set; }
        public bool IsInstalled { get; set; }
        public string Homepage { get; set; }
        public string License { get; set; }
        public string Checksum { get; set; }
        public List<string> Languages { get; set; }
    }
}
