using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Entity with the result of the filter update
    /// </summary>
    public class UpdateResult
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="UpdateResult"/> class.
        /// </summary>
        /// <param name="updatedList">The updated list.</param>
        /// <param name="remainingFiltersCount">The remaining filters count.</param>
        /// <param name="filtersErrors">The filters errors.</param>
        public UpdateResult(List<FullFilterList> updatedList,
            int remainingFiltersCount,
            List<UpdateFilterError> filtersErrors)
        {
            this.updatedList = updatedList;
            this.remainingFiltersCount = remainingFiltersCount;
            this.filtersErrors = filtersErrors;
        }

        /// <summary>
        /// Gets or sets the list of updated filters.
        /// </summary>
        public List<FullFilterList> updatedList { get; set; }

        /// <summary>
        /// Gets or sets the remaining filters count.
        /// </summary>
        public int remainingFiltersCount { get; set; }

        /// <summary>
        /// Gets or sets the filters errors.
        /// </summary>
        public List<UpdateFilterError> filtersErrors { get; set; }
    }
}