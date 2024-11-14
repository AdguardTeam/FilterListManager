using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using FilterListManager;

namespace AdGuard.FilterListManagerProtobuf
{
    public interface IFilterListManager : IDisposable
    {
        FullFilterList InstallCustomFilterList(
            string downloadUrl,
            bool isTrusted,
            [Optional]
            string title,
            [Optional]
            string description);

        long EnableFilterLists(IEnumerable<long> ids, bool isEnabled);
        long InstallFilterLists(IEnumerable<long> ids, bool isInstalled);
        long DeleteCustomFilterLists(IEnumerable<long> ids);
        FullFilterList GetFullFilterListById(long filterId);
        IEnumerable<StoredFilterMetadata> GetStoredFiltersMetadata();
        StoredFilterMetadata GetStoredFilterMetadataById(long filterId);
        void SaveCustomFilterRules(FilterListRules rules);
        void SaveDisabledRules(long filterId, IEnumerable<string> disabledRules);
        UpdateResult UpdateFilters(
            bool ignoreFiltersExpiration,
            int looseTimeout,
            bool ignoreFilterStatus);
        UpdateResult ForceUpdateFiltersByIds(IEnumerable<long> filterIds, int looseTimeout);
        FilterListMetadata FetchFilterListMetadata(string url);
        void LiftUpDatabase();
        IEnumerable<FilterTag> GetAllTags();
        IEnumerable<FilterGroup> GetAllGroups();
        bool ChangeLocale(string suggestedLocale);
        void PullMetadata();
        bool UpdateCustomFilterMetadata(long filterId, string title, bool isTrusted);
        string GetDatabasePath();
        int GetDatabaseVersion();
        FullFilterList InstallCustomFilterFromString(
            string downloadUrl, 
            long lastDownloadTime, 
            bool isEnabled, 
            bool isTrusted, 
            string filterBody, 
            [Optional]
            string customTitle, 
            [Optional]
            string customDescription);
        IEnumerable<ActiveRulesInfo> GetActiveRules();
        IEnumerable<FilterListRulesRaw> GetFilterRulesAsStrings(IEnumerable<long> filterIds);
        void SaveRulesToFileBlob(long filterId, string filePath);
        IEnumerable<DisabledRulesRaw> GetDisabledRules(IEnumerable<long> filterIds);
    }
}