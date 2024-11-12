namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Filter metadata holder
    /// </summary>
    public class FilterListMetadata
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListMetadata"/> class.
        /// </summary>
        /// <param name="title">The title.</param>
        /// <param name="description">The description.</param>
        /// <param name="timeUpdated">The time updated.</param>
        /// <param name="version">The version.</param>
        /// <param name="homepage">The homepage.</param>
        /// <param name="license">The license.</param>
        /// <param name="checksum">The checksum.</param>
        /// <param name="url">The URL.</param>
        /// <param name="rulesCount">The rules count.</param>
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
        /// <summary>
        /// Gets or sets the title.
        /// </summary>
        public string title { get; set; }

        /// <summary>
        /// Gets or sets the description.
        /// </summary>
        public string description { get; set; }

        /// <summary>
        /// Gets or sets the time updated.
        /// </summary>
        public string timeUpdated { get; set; }
        
        /// <summary>
        /// Gets or sets the version.
        /// </summary>
        public string version { get; set; }

        /// <summary>
        /// Gets or sets the homepage.
        /// </summary>
        public string homepage { get; set; }

        /// <summary>
        /// Gets or sets the license.
        /// </summary>
        public string license { get; set; }

        /// <summary>
        /// Gets or sets the checksum.
        /// </summary>
        public string checksum { get; set; }

        /// <summary>
        /// Gets or sets the URL.
        /// </summary>
        public string url { get; set; }

        /// <summary>
        /// Gets or sets the rules count.
        /// </summary>
        public int rulesCount { get; set; }
    }
}