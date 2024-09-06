using System;
using System.Collections.Generic;
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

        /// <summary>
        /// The main setup of the test.
        /// </summary>
        [SetUp]
        public void Setup()
        {
            string coreLibsDllPath = DllProvider.Instance.LibsDllPath;
            Console.WriteLine($"Rust library path is {coreLibsDllPath}");
        }
        
        /// <summary>
        /// Example of a test.
        /// </summary>
        [Test]
        public void CommonTest()
        {
            var manager = new FilterListManager(new Configuration(FilterListType.Standard, m_CurrentDirectory, "en-us",
                10, new List<string>(), "https://filters.adtidy.org/extension/safari/filters.json", "https://filters.adtidy.org/extension/safari/filters_i18n.json", "", 0));

            manager.PullMetadata();
            List<FilterGroup> groups = manager.GetAllGroups();
            Assert.IsTrue(groups.Count > 0);
            List<ActiveRulesInfo> rules = manager.GetActiveRules();
            Assert.IsTrue(rules.Count > 0);
        }
    }
}