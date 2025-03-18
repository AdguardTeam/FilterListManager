namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Raw version of <see cref="FilterListRules"/>
    /// </summary>
    public class FilterListRulesRaw
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListRulesRaw"/> class.
        /// </summary>
        /// <param name="filterId">The filter identifier.</param>
        /// <param name="rules">The rules.</param>
        /// <param name="disabledRules">The disabled rules.</param>
        /// <param name="rulesCount">The rules count.</param>
        public FilterListRulesRaw(int filterId, string rules, string disabledRules, int rulesCount)
        {
            FilterId = filterId;
            Rules = rules;
            DisabledRules = disabledRules;
            RulesCount = rulesCount;
        }
        
        /// <summary>
        /// Gets the filter identifier.
        /// </summary>
        public int FilterId { get; }
        
        /// <summary>
        /// Gets the rules of the filter.
        /// </summary>
        public string Rules { get; }

        /// <summary>
        /// Gets the disabled rules of the filter.
        /// </summary>
        public string DisabledRules { get; }

        /// <summary>
        /// Gets the rules count.
        /// </summary>
        public int RulesCount { get; }
    }
}
