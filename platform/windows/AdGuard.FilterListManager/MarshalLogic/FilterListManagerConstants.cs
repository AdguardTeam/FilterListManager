namespace AdGuard.FilterListManager.MarshalLogic
{
   internal class FilterListManagerConstants
    {
        public FilterListManagerConstants(long userRulesId,
            int customGroupId,
            int specialGroupId,
            long smallestFilterId)
        {
            this.userRulesId = userRulesId;
            this.customGroupId = customGroupId;
            this.specialGroupId = specialGroupId;
            this.smallestFilterId = smallestFilterId;
        }

        public long userRulesId { get; }
        public int customGroupId { get; }
        public int specialGroupId { get; }
        public long smallestFilterId { get; }

        public void Deconstruct(out long userRulesId, out int customGroupId, out int specialGroupId, out long smallestFilterId)
        {
            userRulesId = this.userRulesId;
            customGroupId = this.customGroupId;
            specialGroupId = this.specialGroupId;
            smallestFilterId = this.smallestFilterId;
        }
    }
}
