namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FilterListMetadata
    {
        public FilterListMetadata(string title,
            string description,
            string timeUpdated,
            string version,
            string homepage,
            string license,
            string checksum,
            string url,
            int rulesCount)
        {
            this.title = title;
            this.description = description;
            this.timeUpdated = timeUpdated;
            this.version = version;
            this.homepage = homepage;
            this.license = license;
            this.checksum = checksum;
            this.url = url;
            this.rulesCount = rulesCount;
        }

        public string title { get; set; }
        public string description { get; set; }
        public string timeUpdated { get; set; }
        public string version { get; set; }
        public string homepage { get; set; }
        public string license { get; set; }
        public string checksum { get; set; }
        public string url { get; set; }
        public int rulesCount { get; set; }

        public void Deconstruct(out string title, out string description, out string timeUpdated, out string version, out string homepage, out string license, out string checksum, out string url, out int rulesCount)
        {
            title = this.title;
            description = this.description;
            timeUpdated = this.timeUpdated;
            version = this.version;
            homepage = this.homepage;
            license = this.license;
            checksum = this.checksum;
            url = this.url;
            rulesCount = this.rulesCount;
        }
    }
}