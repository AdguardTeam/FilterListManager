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
        public FilterListRules(long filterId, List<string> rules, List<string> disabledRules)
        {
            this.filterId = filterId;
            this.rules = rules;
            this.disabledRules = disabledRules;
        }

        /// <summary>
        /// Gets or sets the filter identifier.
        /// </summary>
        public long filterId { get; set; }

        /// <summary>
        /// Gets or sets the rules of the filter.
        /// </summary>
        public List<string> rules { get; set; }

        /// <summary>
        /// Gets or sets the disabled rules of the filter.
        /// </summary>
        public List<string> disabledRules { get; set; }
    }
}