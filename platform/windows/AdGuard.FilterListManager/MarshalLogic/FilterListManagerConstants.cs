namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Constants holder form the Rust
    /// </summary>
    public class FilterListManagerConstants
    {
        public FilterListManagerConstants(long userRulesId,
            int customGroupId,
            int specialGroupId,
            long smallestFilterId)
        {
            UserRulesId = userRulesId;
            CustomGroupId = customGroupId;
            SpecialGroupId = specialGroupId;
            SmallestFilterId = smallestFilterId;
        }

        /// <summary>
        /// Gets the user rules identifier.
        /// </summary>
        public long UserRulesId { get; }

        /// <summary>
        /// Gets the custom group identifier.
        /// </summary>
        public int CustomGroupId { get; }

        /// <summary>
        /// Gets the special group identifier.
        /// </summary>
        public int SpecialGroupId { get; }

        /// <summary>
        /// Gets the smallest filter identifier.
        /// </summary>
        public long SmallestFilterId { get; }
    }
}
