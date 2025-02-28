using System;
using System.Collections.Generic;
using System.Diagnostics;
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
            using (IFilterListManager flm = new Api.FilterListManager(serializer))
            {
                flm.Init(configuration);
                flm.PullMetadata();
                flm.UpdateFilters(false, 0, false);
                
                flm.EnableFilterLists(new[] {1, 2, 255}, true);
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
                flm.EnableFilterLists(new[] {1, 2, 255}, true);
                flm.InstallFilterLists(new[] {1, 2, 255}, true);

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
                flm.ForceUpdateFiltersByIds(new[] { 1, 2 }, 0);
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
                FilterListMetadataWithBody filterListMetadataWithBody =
                    flm.FetchFilterListMetadataWithBody("https://filters.adtidy.org/extension/safari/filters/101.txt");
                flm.GetActiveRules();
                flm.DeleteCustomFilterLists(new[] { customFilter.Id });
                string path = flm.GetDatabasePath();
                int version = flm.GetDatabaseVersion();
                flm.SetProxyMode("https://127.0.0.1:8080", RawRequestProxyMode.NoProxy);

                var constants = Api.FilterListManager.SpawnDefaultConstants();

                Debug.Assert(constants.UserRulesId == -2147483648, "UserRulesId must be equal to int::min");
                Debug.Assert(constants.CustomGroupId == -2147483648, "CustomGroupId must be equal to int::min");
                Debug.Assert(constants.SpecialGroupId == 0, "SpecialGroupId must be zero");
                Debug.Assert(constants.SmallestFilterId == -2_000_000_000, "UserRulesId must be two billions");
                Debug.Assert(filterListMetadataWithBody.Metadata.Homepage.Length > 0, "Metadata Homepage must be non-empty");

                Logger.Info("All Ok!");
            }
        }
    }
}