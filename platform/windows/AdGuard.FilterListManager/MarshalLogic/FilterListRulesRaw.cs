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
        public FilterListRulesRaw(int filterId, string rules, string disabledRules)
        {
            FilterId = filterId;
            Rules = rules;
            DisabledRules = disabledRules;
        }
        
        /// <summary>
        /// Gets or sets the filter identifier.
        /// </summary>
        public int FilterId { get; }
        
        /// <summary>
        /// Gets or sets the rules of the filter.
        /// </summary>
        public string Rules { get; }

        /// <summary>
        /// Gets the disabled rules of the filter.
        /// </summary>
        public string DisabledRules { get; }
    }
}
