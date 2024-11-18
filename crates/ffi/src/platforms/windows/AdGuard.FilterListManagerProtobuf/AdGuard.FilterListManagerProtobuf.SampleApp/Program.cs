using System;
using System.Collections.Generic;
using System.IO;
using AdGuard.FilterListManagerProtobuf.Api;
using AdGuard.FilterListManagerProtobuf.Utils;
using AdGuard.Utils.Base.Files;
using AdGuard.Utils.Base.Logging;
using AdGuard.Utils.Base.Logging.TraceListeners;
using AdGuard.Utils.Serializers;
using FilterListManager;

namespace AdGuard.FilterListManagerProtobuf.SampleApp
{
    internal static class Program
    {
        public static void Main(string[] args)
        {
            ITraceListener traceListener = new ColoredConsoleTraceListener();
            Logger.SetCustomListener(traceListener);
            Logger.Info("Hello, I'm filter list manager");
            Logger.Level = LogLevel.Trace;
            FlmDllProvider.SetFlmDllName(Constants.FLM_DLL_NAME);
            FlmDllProvider _ = (FlmDllProvider)FlmDllProvider.Instance;
            Configuration configuration = Api.FilterListManager.SpawnDefaultConfiguration();
            configuration.MetadataUrl = "https://filters.adtidy.org/extension/safari/filters.json";
            configuration.MetadataLocalesUrl = "https://filters.adtidy.org/windows/filters_i18n.json";
            ISerializer<byte[]> serializer = new ProtobufSerializer();
            using (IFilterListManager flm = new Api.FilterListManager(configuration, serializer))
            {
                flm.PullMetadata();
                flm.UpdateFilters(false, 0, false);
                
                flm.EnableFilterLists(new long[] {1, 2, 255}, true);
                FullFilterList customFilter = flm.InstallCustomFilterFromString(
                    string.Empty,
                    1000000000,
                    true,
                    true,
                    "custom filter string body",
                    "custom title",
                    "Desc");
                bool localeResult = flm.ChangeLocale("ru_RU");
                flm.LiftUpDatabase();
                flm.EnableFilterLists(new long[] {1, 2, 255}, true);
                flm.InstallFilterLists(new long[] {1, 2, 255}, true);

                FilterListRules rules1 = new FilterListRules
                {
                    FilterId = customFilter.Id
                };
                rules1.Rules.AddRange(new[]{"hello", "world"});
                flm.SaveCustomFilterRules(rules1);
                flm.SaveDisabledRules(customFilter.Id, new[]{"world"});
                IEnumerable<DisabledRulesRaw> disabledRules1 = 
                    flm.GetDisabledRules(new[] { customFilter.Id });
                IEnumerable<FilterTag> tags = flm.GetAllTags();
                IEnumerable<FilterGroup> groups = flm.GetAllGroups();
                IEnumerable<StoredFilterMetadata> storedFiltersMetadata = flm.GetStoredFiltersMetadata();
                StoredFilterMetadata filterMetadata = flm.GetStoredFilterMetadataById(customFilter.Id);
                
                flm.GetFilterRulesAsStrings(new[] { customFilter.Id });
                string blobPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "flmtest_2.txt");
                FileUtils.Touch(blobPath);
                flm.SaveRulesToFileBlob(customFilter.Id, blobPath);
                flm.GetFullFilterListById(customFilter.Id);
                flm.ForceUpdateFiltersByIds(new long[] { 1, 2 }, 0);
                customFilter = flm.InstallCustomFilterList(
                    "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt",
                    true,
                    "title",
                    "description");
                flm.UpdateCustomFilterMetadata(
                    customFilter.Id,
                    "new title",
                       true);
                FilterListMetadata filterListMetadata =
                    flm.FetchFilterListMetadata("https://filters.adtidy.org/extension/safari/filters/101.txt");
                flm.GetActiveRules();
                flm.DeleteCustomFilterLists(new[] { customFilter.Id });
                string path = flm.GetDatabasePath();
                int version = flm.GetDatabaseVersion();
            }
        }
    }
}