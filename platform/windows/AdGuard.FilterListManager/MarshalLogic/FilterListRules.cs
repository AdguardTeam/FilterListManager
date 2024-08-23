using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FilterListRules
    {
        public FilterListRules(long filterId, List<string> rules, List<string> disabledRules)
        {
            this.filterId = filterId;
            this.rules = rules;
            this.disabledRules = disabledRules;
        }

        public long filterId { get; set; }
        public List<string> rules { get; set; }
        public List<string> disabledRules { get; set; }

        public void Deconstruct(out long filterId, out List<string> rules, out List<string> disabledRules)
        {
            filterId = this.filterId;
            rules = this.rules;
            disabledRules = this.disabledRules;
        }
    }
}