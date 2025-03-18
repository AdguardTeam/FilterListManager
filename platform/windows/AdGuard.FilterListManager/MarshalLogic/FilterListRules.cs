using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Filter rules holder class
    /// </summary>
    public class FilterListRules
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListRules"/> class.
        /// </summary>
        /// <param name="filterId">The filter identifier.</param>
        /// <param name="rules">The rules.</param>
        /// <param name="disabledRules">The disabled rules.</param>
        /// <param name="rulesCount">The rules count.</param>
        public FilterListRules(int filterId, List<string> rules, List<string> disabledRules, int rulesCount)
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
        public List<string> Rules { get; }

        /// <summary>
        /// Gets the disabled rules of the filter.
        /// </summary>
        public List<string> DisabledRules { get; }

        /// <summary>
        /// Gets the rules count.
        /// </summary>
        public int RulesCount { get; }
    }
}