using System;
using FilterListManager;
using AdGuard.FilterListManagerProtobuf.RustInterface;
using Google.Protobuf;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using AdGuard.FilterListManagerProtobuf.ProtobufGeneratedImpl;
using AdGuard.Utils.Base.Logging;
using AdGuard.Utils.Serializers;

namespace AdGuard.FilterListManagerProtobuf
{
    public class FilterListManager : IFilterListManager
    {
        private readonly IntPtr m_FLMHandle;
        private readonly ISerializer<byte[]> m_Serializer;

        /// <summary>
        ///  header - crates/ffi/src/flm_native_interface.h
        ///  swift.file - crates/ffi/src/platforms/apple/flmctest/flmctest/main.swift
        ///  protobuf schema - crates/ffi/src/protobuf
        /// </summary>
        /// Spawns configuration for set up
        public static Configuration SpawnDefaultConfiguration()
        {
            return ProtobufBridge.MakeDefaultConfiguration();
        }

        public FilterListManager(Configuration configuration, ISerializer<byte[]> serializer)
        {
            m_FLMHandle = ProtobufBridge.InitFLM(configuration);
            m_Serializer = serializer;
        }

        #region IFilterListManager members

        public FullFilterList InstallCustomFilterList(
            string downloadUrl,
            bool isTrusted,
            [Optional]
            string title,
            [Optional]
            string description)
        {
            InstallCustomFilterListRequest request = new InstallCustomFilterListRequest
            {
                DownloadUrl = downloadUrl,
                IsTrusted = isTrusted,
                Title = title,
                Description = description
            };

            InstallCustomFilterListResponse response = 
                CallRust<InstallCustomFilterListResponse>(request);
            return response.FilterList;
        }

        public long EnableFilterLists(IEnumerable<long> ids, bool isEnabled) 
        {
            EnableFilterListsRequest request = new EnableFilterListsRequest
            {
                IsEnabled = isEnabled
            };

            request.Ids.AddRange(ids);
            EnableFilterListsResponse response = CallRust<EnableFilterListsResponse>(request);
            return response.Count;
        }

        public long InstallFilterLists(IEnumerable<long> ids, bool isInstalled)
        {
            InstallFilterListsRequest request = new InstallFilterListsRequest
            {
                IsInstalled = isInstalled
            };

            request.Ids.AddRange(ids);
            InstallFilterListsResponse response = 
                CallRust<InstallFilterListsResponse>(request);
            return response.Count;
        }

        public long DeleteCustomFilterLists(IEnumerable<long> ids)
        {
            DeleteCustomFilterListsRequest request = new DeleteCustomFilterListsRequest();
            request.Ids.AddRange(ids);
            DeleteCustomFilterListsResponse response = 
                CallRust<DeleteCustomFilterListsResponse>(request);
            return response.Count;
        }

        public FullFilterList GetFullFilterListById(long filterId)
        {
            GetFullFilterListByIdRequest request = new GetFullFilterListByIdRequest
            {
                Id = filterId
            };

            GetFullFilterListByIdResponse response = CallRust<GetFullFilterListByIdResponse>(request);
            return response.FilterList;
        }

        public IEnumerable<StoredFilterMetadata> GetStoredFiltersMetadata()
        {
            EmptyRequest request = new EmptyRequest();
            GetStoredFiltersMetadataResponse response = 
                CallRust<GetStoredFiltersMetadataResponse>(request);
            return response.FilterLists;
        }

        public StoredFilterMetadata GetStoredFilterMetadataById(long filterId)
        {
            GetStoredFiltersMetadataByIdRequest request = new GetStoredFiltersMetadataByIdRequest
            {
                Id = filterId
            };

            GetStoredFilterMetadataByIdResponse response = 
                CallRust<GetStoredFilterMetadataByIdResponse>(request);
            return response.FilterList;
        }

        public void SaveCustomFilterRules(FilterListRules rules)
        {
            SaveCustomFilterRulesRequest request = new SaveCustomFilterRulesRequest
            {
                Rules = rules
            };

            CallRust<EmptyResponse>(request);
        }

        public void SaveDisabledRules(long filterId, IEnumerable<string> disabledRules)
        {
            SaveDisabledRulesRequest request = new SaveDisabledRulesRequest
            {
                FilterId = filterId
            };

            request.DisabledRules.AddRange(disabledRules);
            CallRust<EmptyResponse>(request);
        }

        public UpdateResult UpdateFilters(
            bool ignoreFiltersExpiration,
            int looseTimeout,
            bool ignoreFilterStatus)
        {
            UpdateFiltersRequest request = new UpdateFiltersRequest
            {
                IgnoreFiltersExpiration = ignoreFiltersExpiration,
                LooseTimeout = looseTimeout,
                IgnoreFiltersStatus = ignoreFilterStatus
            };

            UpdateFiltersResponse response = CallRust<UpdateFiltersResponse>(request);
            return response.Result;
        }

        public UpdateResult ForceUpdateFiltersByIds(IEnumerable<long> filterIds, int looseTimeout)
        {
            ForceUpdateFiltersByIdsRequest request = new ForceUpdateFiltersByIdsRequest
            {
                LooseTimeout = looseTimeout,
            };
            
            request.Ids.AddRange(filterIds);
            ForceUpdateFiltersByIdsResponse response = 
                CallRust<ForceUpdateFiltersByIdsResponse>(request);
            return response.Result;
        }

        public FilterListMetadata FetchFilterListMetadata(string url)
        {
            FetchFilterListMetadataRequest request = new FetchFilterListMetadataRequest
            {
                Url = url,
            };
            
            FetchFilterListMetadataResponse response = 
                CallRust<FetchFilterListMetadataResponse>(request);
            return response.Metadata;
        }

        public void LiftUpDatabase()
        {
            EmptyRequest request = new EmptyRequest();
            CallRust<EmptyResponse>(request);
        }

        public IEnumerable<FilterTag> GetAllTags()
        {
            EmptyRequest request = new EmptyRequest();
            GetAllTagsResponse response = CallRust<GetAllTagsResponse>(request);
            return response.Tags;
        }

        public IEnumerable<FilterGroup> GetAllGroups()
        {
            EmptyRequest request = new EmptyRequest();
            GetAllGroupsResponse response = CallRust<GetAllGroupsResponse>(request);
            return response.Groups;
        }

        public bool ChangeLocale(string suggestedLocale)
        {
            ChangeLocaleRequest request = new ChangeLocaleRequest
            {
                SuggestedLocale = suggestedLocale
            };
            
            ChangeLocaleResponse response = CallRust<ChangeLocaleResponse>(request);
            return response.Success;
        }

        public void PullMetadata()
        {
            EmptyRequest request = new EmptyRequest();
            CallRust<EmptyResponse>(request);
        }

        public bool UpdateCustomFilterMetadata(long filterId, string title, bool isTrusted)
        {
            UpdateCustomFilterMetadataRequest request = new UpdateCustomFilterMetadataRequest
            {
                FilterId = filterId,
                Title = title,
                IsTrusted = isTrusted
            };
            
            UpdateCustomFilterMetadataResponse response = CallRust<UpdateCustomFilterMetadataResponse>(request);
            return response.Success;
        }

        public string GetDatabasePath()
        {
            EmptyRequest request = new EmptyRequest();
            GetDatabasePathResponse response = CallRust<GetDatabasePathResponse>(request);
            return response.Path;
        }

        public int GetDatabaseVersion()
        {
            EmptyRequest request = new EmptyRequest();
            GetDatabaseVersionResponse response = CallRust<GetDatabaseVersionResponse>(request);
            return response.Version;
        }

        public FullFilterList InstallCustomFilterFromString(
            string downloadUrl, 
            long lastDownloadTime, 
            bool isEnabled, 
            bool isTrusted,
            string filterBody, 
            [Optional]
            string customTitle, 
            [Optional]
            string customDescription)
        {
            InstallCustomFilterFromStringRequest request = new InstallCustomFilterFromStringRequest
            {
                DownloadUrl = downloadUrl,
                LastDownloadTime = lastDownloadTime,
                IsEnabled = isEnabled,
                IsTrusted = isTrusted,
                FilterBody = filterBody,
                CustomTitle = customTitle,
                CustomDescription = customDescription
            };
            
            InstallCustomFilterFromStringResponse response = CallRust<InstallCustomFilterFromStringResponse>(request);
            return response.FilterList;
        }

        public IEnumerable<ActiveRulesInfo> GetActiveRules()
        {
            EmptyRequest request = new EmptyRequest();
            GetActiveRulesResponse response = CallRust<GetActiveRulesResponse>(request);
            return response.Rules;
        }

        public IEnumerable<FilterListRulesRaw> GetFilterRulesAsStrings(IEnumerable<long> filterIds)
        {
            GetFilterRulesAsStringsRequest request = new GetFilterRulesAsStringsRequest();
            request.Ids.AddRange(filterIds);
            GetFilterRulesAsStringsResponse response = CallRust<GetFilterRulesAsStringsResponse>(request);
            return response.RulesList;
        }

        public void SaveRulesToFileBlob(long filterId, string filePath)
        {
            SaveRulesToFileBlobRequest request = new SaveRulesToFileBlobRequest();
            CallRust<EmptyResponse>(request);
        }

        public IEnumerable<DisabledRulesRaw> GetDisabledRules(IEnumerable<long> filterIds)
        {
            GetDisabledRulesRequest request = new GetDisabledRulesRequest();
            request.Ids.AddRange(filterIds);
            GetDisabledRulesResponse response = CallRust<GetDisabledRulesResponse>(request);
            return response.RulesRaw;
        }

        #endregion


        #region Helpers

        private TMessage CallRust<TMessage>(
            IMessage message,
            [CallerMemberName] string methodName = null) 
            where TMessage : IMessage, IAGOuterError
        {
            FFIMethod method = GetMethod(methodName);
            byte[] args = message.ToByteArray();
            byte[] data = ProtobufBridge.CallRust(m_FLMHandle, method, args);
            TMessage response = m_Serializer.DeserializeObject<TMessage>(data);
            if (response.Error != null)
            {
                throw new AGOuterException(response.Error);
            }

            return response;
        }

        private FFIMethod GetMethod(string methodName)
        {
            if (Enum.TryParse(methodName, out FFIMethod method))
            {
                Logger.Verbose("Parsed method: {0}->{1}", 
                    methodName,
                    method);
                return method;
            }

            throw new InvalidCastException($"Failed to parse {methodName}");
        }

        #endregion


        #region IDisposable members

        ~FilterListManager()
        {
            Dispose(false);
        }
        
        private void ReleaseManagedResources()
        {
        }

        private void ReleaseUnmanagedResources()
        {
            ProtobufBridge.FreeFLMHandle(m_FLMHandle);
        }

        private void Dispose(bool disposing)
        {
            if (disposing)
            {
                ReleaseManagedResources();
            }
            
            ReleaseUnmanagedResources();
        }

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        #endregion

        
    }
}
