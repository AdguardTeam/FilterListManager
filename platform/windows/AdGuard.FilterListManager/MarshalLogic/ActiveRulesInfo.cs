using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class ActiveRulesInfo
    {
        public long FilterId { get; }
        public int GroupId { get; }
        public bool IsTrusted { get; }
        public List<string> Rules { get; }

        public ActiveRulesInfo(long filterId, int groupId, bool isTrusted, List<string> rules)
        {
            FilterId = filterId;
            GroupId = groupId;
            IsTrusted = isTrusted;
            Rules = rules;
        }
    }
}
