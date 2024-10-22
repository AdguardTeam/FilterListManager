namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FilterListRulesRaw
    {
        public FilterListRulesRaw(long filterId, string rules, string disabledRules)
        {
            FilterId = filterId;
            Rules = rules;
            DisabledRules = disabledRules;
        }

        public long FilterId { get; }
        public string Rules { get; }
        public string DisabledRules { get; }
    }
}
