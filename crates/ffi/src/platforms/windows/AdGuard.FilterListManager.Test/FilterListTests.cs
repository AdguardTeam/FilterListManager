using System;
using System.Collections.Generic;
using System.IO;
using AdGuard.FilterListManager.RustInterface;
using AdGuard.FilterListManager.Utils;
using AdGuard.Utils.Base.Files;
using AdGuard.Utils.Base.Logging.TraceListeners;
using AdGuard.Utils.Base.Logging;
using AdGuard.Utils.Serializers;
using FilterListManager;
using NUnit.Framework;
using NUnit.Framework.Internal;
using Logger = AdGuard.Utils.Base.Logging.Logger;

namespace AdGuard.FilterListManager.Test
{
    /// <summary>
    /// Example of filter list tests
    /// </summary>
    public class FilterListTests
    {
        private const int REQUEST_TIMEOUT_MS = 60 * 1000;
        private readonly string m_CurrentDirectory = AppDomain.CurrentDomain.BaseDirectory;
        private ProtobufSerializer m_Serializer;

        /// <summary>
        /// The main setup of the test.
        /// </summary>
        [SetUp]
        public void Setup()
        {
            string coreLibsDllPath = FilterManagerDllProvider.Instance.LibsDllPath;
            Console.WriteLine($"Rust library path is {coreLibsDllPath}");
            FileUtils.DeleteQuietly(Path.Combine(m_CurrentDirectory, "agflm_standard.db"));
            ITraceListener traceListener = new TestContextTraceListener(TestExecutionContext.CurrentContext);
            Logger.SetCustomListener(traceListener);
            Logger.Info("Hello, I'm filter list manager");
            Logger.Level = LogLevel.Trace;
            m_Serializer = new ProtobufSerializer();
        }

        /// <summary>
        /// Example of a test.
        /// </summary>
        [Test]
        public void ErrorHandlingTest()
        {
            using (IFilterListManager flm = new LocalFilterListManager(m_Serializer))
            {
                Configuration configuration = flm.SpawnDefaultConfiguration();
                configuration.MetadataUrl =
                    "https://filters.adtidy.org/windows/filters.json";
                configuration.MetadataLocalesUrl =
                    "https://filters.adtidy.org/windows/filters_i18n.json";
                configuration.AutoLiftUpDatabase = true;
                configuration.AppName = "AdGuard.FilterListManager.Test";
                configuration.Version = "1.0";
                configuration.DefaultFilterListExpiresPeriodSec = 10;
                configuration.ShouldIgnoreExpiresForLocalUrls = true;
                configuration.FiltersCompilationPolicy = new FiltersCompilationPolicy
                {
                    Constants = { "windows_is_the_best" }
                };
                flm.Init(configuration);
                flm.PullMetadata();
            }
        }

        /// <summary>
        /// Example of a test.
        /// </summary>
        [Test]
        public void CommonTest()
        {
            using (IFilterListManager flm = new FilterListManager(m_Serializer))
            {
                Configuration configuration = flm.SpawnDefaultConfiguration();
                configuration.MetadataUrl = "https://filters.adtidy.org/windows/filters.json";
                configuration.MetadataLocalesUrl = "https://filters.adtidy.org/windows/filters_i18n.json";
                configuration.AutoLiftUpDatabase = true;
                configuration.AppName = "AdGuard.FilterListManager.Test";
                configuration.Version = "1.0";
                configuration.DefaultFilterListExpiresPeriodSec = 10;
                configuration.FiltersCompilationPolicy = new FiltersCompilationPolicy
                {
                    Constants = { "windows_is_the_best" }
                };
                flm.Init(configuration);
                flm.PullMetadata();
                flm.UpdateFilters(false, REQUEST_TIMEOUT_MS, false);

                flm.EnableFilterLists(new[] { 1, 2, 255 }, true);
                FullFilterList customFilter = flm.InstallCustomFilterFromString(
                    string.Empty,
                    1000000000,
                    true,
                    true,
                    "custom filter string body",
                    "custom title",
                    "Desc");
                bool localeResult = flm.ChangeLocale("ru_RU");
                //flm.LiftUpDatabase();
                flm.EnableFilterLists(new[] { 1, 2, 255 }, true);
                flm.InstallFilterLists(new[] { 1, 2, 255 }, true);
                IEnumerable<RulesCountByFilter> rulesCount = flm.GetRulesCount(new[] { 1, 2, 255 });

                FilterListRules rules1 = new FilterListRules
                {
                    FilterId = customFilter.Id
                };
                rules1.Rules.AddRange(new[] { "hello", "world" });
                flm.SaveCustomFilterRules(rules1);
                flm.SaveDisabledRules(customFilter.Id, new[] { "world" });
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
                flm.ForceUpdateFiltersByIds(new[] { 1, 2 }, REQUEST_TIMEOUT_MS);
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
                flm.GetActiveRulesRaw(new[] { customFilter.Id });
                flm.DeleteCustomFilterLists(new[] { customFilter.Id });
                string path = flm.GetDatabasePath();
                int version = flm.GetDatabaseVersion();
                flm.SetProxyMode("https://127.0.0.1:8080", RawRequestProxyMode.NoProxy);

                FLMConstants constants = FilterListManager.SpawnDefaultConstants();

                Assert.AreEqual(-2147483648, constants.UserRulesId, "UserRulesId must be equal to int::min");
                Assert.AreEqual(-2147483648, constants.CustomGroupId, "CustomGroupId must be equal to int::min");
                Assert.AreEqual(0, constants.SpecialGroupId, "SpecialGroupId must be zero");
                Assert.AreEqual(-2_000_000_000, constants.SmallestFilterId, "UserRulesId must be two billions");
                Assert.IsTrue(filterListMetadataWithBody.Metadata.Homepage.Length > 0, "Metadata Homepage must be non-empty");

                Logger.Info("All Ok!");
            }
        }
    }
}