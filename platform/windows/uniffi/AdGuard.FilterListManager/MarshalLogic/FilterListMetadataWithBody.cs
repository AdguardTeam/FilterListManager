namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Filter metadata holder
    /// </summary>
    public class FilterListMetadataWithBody
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListMetadataWithBody"/> class.
        /// </summary>
        /// <param name="metadata">The filter metadata.</param>
        /// <param name="filterBody">The filter body.</param>
        public FilterListMetadataWithBody(FilterListMetadata metadata, string filterBody)
        {
            this.Metadata = metadata;
            this.FilterBody = filterBody;
        }
        /// <summary>
        /// Gets or sets the filter metadata.
        /// </summary>
        public FilterListMetadata Metadata { get; set; }

        /// <summary>
        /// Gets or sets the filter body.
        /// </summary>
        public string FilterBody { get; set; }
    }
}