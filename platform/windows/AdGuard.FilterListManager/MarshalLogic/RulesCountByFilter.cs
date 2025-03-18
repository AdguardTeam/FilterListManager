namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Rules count in this filter list. Simply a number of non-empty lines
    /// and does not start with a comment marker.
    /// </summary>
    public class RulesCountByFilter
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="RulesCountByFilter"/> class.
        /// </summary>
        /// <param name="filterId">The filter identifier.</param>
        /// <param name="rulesCount">The rules count.</param>
        public RulesCountByFilter(int filterId, int rulesCount)
        {
            FilterId = filterId;
            RulesCount = rulesCount;
        }

        /// <summary>
        /// Gets the filter identifier.
        /// </summary>
        public int FilterId { get; }

        /// <summary>
        /// Gets the rules count.
        /// </summary>
        public int RulesCount { get; }
    }
}