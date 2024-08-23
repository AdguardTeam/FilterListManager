using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FullFilterList
    {
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

        public long id { get; set; }
        public int groupId { get; set; }
        public long timeUpdated { get; set; }
        public long lastDownloadTime { get; set; }
        public string title { get; set; }
        public string description { get; set; }
        public string version { get; set; }
        public int displayNumber { get; set; }
        public string downloadUrl { get; set; }
        public string subscriptionUrl { get; set; }
        public List<FilterTag> tags { get; set; }
        public int expires { get; set; }
        public bool isTrusted { get; set; }
        public bool isCustom { get; set; }
        public bool isEnabled { get; set; }
        public bool isInstalled { get; set; }
        public string homepage { get; set; }
        public string license { get; set; }
        public string checksum { get; set; }
        public List<string> languages { get; set; }
        public FilterListRules rules { get; set; }

        public void Deconstruct(out long id, out int groupId, out long timeUpdated, out long lastDownloadTime, out string title, out string description, out string version, out int displayNumber, out string downloadUrl, out string subscriptionUrl, out List<FilterTag> tags, out int expires, out bool isTrusted, out bool isCustom, out bool isEnabled, out bool isInstalled, out string homepage, out string license, out string checksum, out List<string> languages, out FilterListRules rules)
        {
            id = this.id;
            groupId = this.groupId;
            timeUpdated = this.timeUpdated;
            lastDownloadTime = this.lastDownloadTime;
            title = this.title;
            description = this.description;
            version = this.version;
            displayNumber = this.displayNumber;
            downloadUrl = this.downloadUrl;
            subscriptionUrl = this.subscriptionUrl;
            tags = this.tags;
            expires = this.expires;
            isTrusted = this.isTrusted;
            isCustom = this.isCustom;
            isEnabled = this.isEnabled;
            isInstalled = this.isInstalled;
            homepage = this.homepage;
            license = this.license;
            checksum = this.checksum;
            languages = this.languages;
            rules = this.rules;
        }
    }
}