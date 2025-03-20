namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Holder for filter tag entity.
    /// </summary>
    public class FilterTag
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterTag"/> class.
        /// </summary>
        /// <param name="id">The tag identifier.</param>
        /// <param name="keyword">The keyword itself.</param>
        public FilterTag(int id, string keyword)
        {
            this.id = id;
            this.keyword = keyword;
        }

        /// <summary>
        /// Gets or sets the tag identifier.
        /// </summary>
        public int id { get; set; }

        /// <summary>
        /// Gets or sets the keyword (the tag itself).
        /// </summary>
        public string keyword { get; set; }
    }
}