using System;
using System.Collections.Generic;
using System.Linq;
using AdGuard.FilterListManager.MarshalLogic;
using AdGuard.FilterListManager.Utils;
using NUnit.Framework;

namespace AdGuard.FilterListManager.Test
{
    /// <summary>
    /// Example of filter list tests
    /// </summary>
    public class FilterListTests
    {
        private readonly string m_CurrentDirectory = AppDomain.CurrentDomain.BaseDirectory;
        private const int REQUEST_TIMEOUT_MS = 60 * 1000;

        /// <summary>
        /// The main setup of the test.
        /// </summary>
        [SetUp]
        public void Setup()
        {
            string coreLibsDllPath = FilterManagerDllProvider.Instance.LibsDllPath;
            Console.WriteLine($"Rust library path is {coreLibsDllPath}");
        }

        /// <summary>
        /// Example of a test.
        /// </summary>
        [Test]
        public void CommonTest()
        {
            var manager = new FilterListManager(new Configuration(
                FilterListType.Standard, 
                m_CurrentDirectory,
                "en-us",
                10,
                new List<string>(), 
                "https://filters.adtidy.org/extension/safari/filters.json",
                "https://filters.adtidy.org/extension/safari/filters_i18n.json", 
                "", 
                0,
                true));
            //manager.LiftUpDatabase(); //TODO https://jira.int.agrd.dev/browse/AG-36219
            manager.PullMetadata();
            List<StoredFilterMetadata> metas = manager.GetStoredFiltersMetadata();
            Assert.IsTrue(metas.Count > 0);

            StoredFilterMetadata firstFilter = metas.FirstOrDefault(a => a.Id == 1);
            Assert.IsNotNull(firstFilter);
            StoredFilterMetadata meta = manager.GetStoredFiltersMetadataById(firstFilter.Id);
            Assert.IsNotNull(meta);
            List<FilterGroup> groups = manager.GetAllGroups();
            Assert.IsTrue(groups.Count > 0);
            List<ActiveRulesInfo> rules = manager.GetActiveRules();
            Assert.IsTrue(rules.Count > 0);

            manager.UpdateFilters(true, REQUEST_TIMEOUT_MS, true);
            List<FilterListRulesRaw> rulesRaw = manager.GetFilterRulesAsStrings(new List<long>{ firstFilter.Id });
            Assert.IsTrue(rulesRaw.Count > 0);

        }
    }
}