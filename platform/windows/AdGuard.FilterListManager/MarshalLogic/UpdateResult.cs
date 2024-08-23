using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public class UpdateResult
    {
        public UpdateResult(List<FullFilterList> updatedList,
            int remainingFiltersCount,
            List<UpdateFilterError> filtersErrors)
        {
            this.updatedList = updatedList;
            this.remainingFiltersCount = remainingFiltersCount;
            this.filtersErrors = filtersErrors;
        }

        public List<FullFilterList> updatedList { get; set; }
        public int remainingFiltersCount { get; set; }
        public List<UpdateFilterError> filtersErrors { get; set; }

        public void Deconstruct(out List<FullFilterList> updatedList, out int remainingFiltersCount, out List<UpdateFilterError> filtersErrors)
        {
            updatedList = this.updatedList;
            remainingFiltersCount = this.remainingFiltersCount;
            filtersErrors = this.filtersErrors;
        }
    }
}