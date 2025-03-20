namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// The group of filters holder class.
    /// </summary>
    public class FilterGroup
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterGroup"/> class.
        /// </summary>
        /// <param name="id">The identifier.</param>
        /// <param name="name">The name.</param>
        /// <param name="displayNumber">The display number.</param>
        public FilterGroup(int id, string name, int displayNumber)
        {
            this.id = id;
            this.name = name;
            this.displayNumber = displayNumber;
        }

        /// <summary>
        /// Gets or sets the filter group.
        /// </summary>
        public int id { get; set; }

        /// <summary>
        /// Gets or sets the group name.
        /// </summary>
        public string name { get; set; }

        /// <summary>
        /// Gets or sets the display number of the group.
        /// </summary>
        public int displayNumber { get; set; }
    }
}