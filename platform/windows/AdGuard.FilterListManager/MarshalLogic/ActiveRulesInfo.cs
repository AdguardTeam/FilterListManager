using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Enabled rules holder for one filter.
    /// </summary>
    public class ActiveRulesInfo
    {
        /// <summary>
        /// Gets the filter identifier.
        /// </summary>
        public int FilterId { get; }

        /// <summary>
        /// Gets the group identifier.
        /// </summary>
        public int GroupId { get; }

        /// <summary>
        /// Gets a value indicating whether this filter is trusted.
        /// </summary>
        /// <value>
        ///   <c>true</c> if this filter is trusted; otherwise, <c>false</c>.
        /// </value>
        public bool IsTrusted { get; }

        /// <summary>
        /// Gets the rules of the filter.
        /// </summary>
        public List<string> Rules { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="ActiveRulesInfo"/> class.
        /// </summary>
        /// <param name="filterId">The filter identifier.</param>
        /// <param name="groupId">The group identifier.</param>
        /// <param name="isTrusted">if set to <c>true</c> this filter is trusted.</param>
        /// <param name="rules">The rules of the filter.</param>
        public ActiveRulesInfo(int filterId, int groupId, bool isTrusted, List<string> rules)
        {
            FilterId = filterId;
            GroupId = groupId;
            IsTrusted = isTrusted;
            Rules = rules;
        }
    }
}
