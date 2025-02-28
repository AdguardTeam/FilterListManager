using System;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using AdGuard.FilterListManagerProtobuf.Api.Exceptions;
using AdGuard.FilterListManagerProtobuf.RustInterface;
using AdGuard.Utils.Base.Logging;
using AdGuard.Utils.Serializers;
using FilterListManager;
using Google.Protobuf;

namespace AdGuard.FilterListManagerProtobuf.Api
{
    public class FilterListManager : IFilterListManager
    {
        private IntPtr m_FLMHandle;
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

        /// <summary>
        /// Spawns a struct of Filter List Manager public constants 
        /// </summary>
        /// <returns></returns>
        public static FLMConstants SpawnDefaultConstants()
        {
            return ProtobufBridge.GetFLMConstants();
        }

        public FilterListManager(ISerializer<byte[]> serializer)
        {
            m_Serializer = serializer;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void Init(Configuration configuration)
        {
            m_FLMHandle = ProtobufBridge.InitFLM(configuration);
        }

        #region IFilterListManager members
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
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
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public long EnableFilterLists(IEnumerable<int> ids, bool isEnabled) 
        {
            EnableFilterListsRequest request = new EnableFilterListsRequest
            {
                IsEnabled = isEnabled
            };

            request.Ids.AddRange(ids);
            EnableFilterListsResponse response = CallRust<EnableFilterListsResponse>(request);
            return response.Count;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public long InstallFilterLists(IEnumerable<int> ids, bool isInstalled)
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
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public long DeleteCustomFilterLists(IEnumerable<int> ids)
        {
            DeleteCustomFilterListsRequest request = new DeleteCustomFilterListsRequest();
            request.Ids.AddRange(ids);
            DeleteCustomFilterListsResponse response = 
                CallRust<DeleteCustomFilterListsResponse>(request);
            return response.Count;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public FullFilterList GetFullFilterListById(int filterId)
        {
            GetFullFilterListByIdRequest request = new GetFullFilterListByIdRequest
            {
                Id = filterId
            };

            GetFullFilterListByIdResponse response = CallRust<GetFullFilterListByIdResponse>(request);
            return response.FilterList;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<StoredFilterMetadata> GetStoredFiltersMetadata()
        {
            EmptyRequest request = new EmptyRequest();
            GetStoredFiltersMetadataResponse response = 
                CallRust<GetStoredFiltersMetadataResponse>(request);
            return response.FilterLists;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public StoredFilterMetadata GetStoredFilterMetadataById(int filterId)
        {
            GetStoredFiltersMetadataByIdRequest request = new GetStoredFiltersMetadataByIdRequest
            {
                Id = filterId
            };

            GetStoredFilterMetadataByIdResponse response = 
                CallRust<GetStoredFilterMetadataByIdResponse>(request);
            return response.FilterList;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void SaveCustomFilterRules(FilterListRules rules)
        {
            SaveCustomFilterRulesRequest request = new SaveCustomFilterRulesRequest
            {
                Rules = rules
            };

            CallRust<EmptyResponse>(request);
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void SaveDisabledRules(int filterId, IEnumerable<string> disabledRules)
        {
            SaveDisabledRulesRequest request = new SaveDisabledRulesRequest
            {
                FilterId = filterId
            };

            request.DisabledRules.AddRange(disabledRules);
            CallRust<EmptyResponse>(request);
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
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
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public UpdateResult ForceUpdateFiltersByIds(IEnumerable<int> filterIds, int looseTimeout)
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
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
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
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public FilterListMetadataWithBody FetchFilterListMetadataWithBody(string url)
        {
            FetchFilterListMetadataWithBodyRequest request = new FetchFilterListMetadataWithBodyRequest
            {
                Url = url,
            };

            FetchFilterListMetadataWithBodyResponse response =
                CallRust<FetchFilterListMetadataWithBodyResponse>(request);
            return response.Metadata;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void LiftUpDatabase()
        {
            EmptyRequest request = new EmptyRequest();
            CallRust<EmptyResponse>(request);
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<FilterTag> GetAllTags()
        {
            EmptyRequest request = new EmptyRequest();
            GetAllTagsResponse response = CallRust<GetAllTagsResponse>(request);
            return response.Tags;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<FilterGroup> GetAllGroups()
        {
            EmptyRequest request = new EmptyRequest();
            GetAllGroupsResponse response = CallRust<GetAllGroupsResponse>(request);
            return response.Groups;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public bool ChangeLocale(string suggestedLocale)
        {
            ChangeLocaleRequest request = new ChangeLocaleRequest
            {
                SuggestedLocale = suggestedLocale
            };
            
            ChangeLocaleResponse response = CallRust<ChangeLocaleResponse>(request);
            return response.Success;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void PullMetadata()
        {
            EmptyRequest request = new EmptyRequest();
            CallRust<EmptyResponse>(request);
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public bool UpdateCustomFilterMetadata(int filterId, string title, bool isTrusted)
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
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public string GetDatabasePath()
        {
            EmptyRequest request = new EmptyRequest();
            GetDatabasePathResponse response = CallRust<GetDatabasePathResponse>(request);
            return response.Path;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public int GetDatabaseVersion()
        {
            EmptyRequest request = new EmptyRequest();
            GetDatabaseVersionResponse response = CallRust<GetDatabaseVersionResponse>(request);
            return response.Version;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
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
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<ActiveRulesInfo> GetActiveRules()
        {
            EmptyRequest request = new EmptyRequest();
            GetActiveRulesResponse response = CallRust<GetActiveRulesResponse>(request);
            return response.Rules;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<FilterListRulesRaw> GetFilterRulesAsStrings(IEnumerable<int> filterIds)
        {
            GetFilterRulesAsStringsRequest request = new GetFilterRulesAsStringsRequest();
            request.Ids.AddRange(filterIds);
            GetFilterRulesAsStringsResponse response = CallRust<GetFilterRulesAsStringsResponse>(request);
            return response.RulesList;
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void SaveRulesToFileBlob(int filterId, string filePath)
        {
            SaveRulesToFileBlobRequest request = new SaveRulesToFileBlobRequest
            {
                FilterId = filterId,
                FilePath = filePath
            };
            
            CallRust<EmptyResponse>(request);
        }
        
        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<DisabledRulesRaw> GetDisabledRules(IEnumerable<int> filterIds)
        {
            GetDisabledRulesRequest request = new GetDisabledRulesRequest();
            request.Ids.AddRange(filterIds);
            GetDisabledRulesResponse response = CallRust<GetDisabledRulesResponse>(request);
            return response.RulesRaw;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void SetProxyMode(string customProxyAddr,  RawRequestProxyMode proxyMode)
        {
            SetProxyModeRequest request = new SetProxyModeRequest
            {
                CustomProxyAddr = customProxyAddr,
                Mode = proxyMode
            };
            
            CallRust<EmptyResponse>(request);
        }

        #endregion


        #region Helpers

        private TMessage CallRust<TMessage>(
            IMessage message,
            [CallerMemberName] string methodName = null) 
            where TMessage : IMessage, IAGOuterError
        {
            string errorTemplate = $"Cannot call RUST method \"{methodName}\"";
            if (m_FLMHandle == IntPtr.Zero)
            {
                string errorMessage = $"instance must be initialized with \"{nameof(Init)}\" before invocation";
                Logger.Error(errorTemplate);
                throw new FilterListManagerNotInitializedException(errorMessage);
            }

            try
            {
                FfiMethod method = GetMethod(methodName);
                byte[] args = message.ToByteArray();
                byte[] data = ProtobufBridge.CallRust(m_FLMHandle, method, args);
                TMessage response = m_Serializer.DeserializeObject<TMessage>(data);
                if (response.Error != null)
                {
                    throw new AgOuterException(response.Error);
                }

                return response;
            }
            catch (AgOuterException)
            {
                throw;
            }
            catch (Exception ex)
            {
                throw new FilterListManagerCommonException(errorTemplate, ex);
            }
        }

        private FfiMethod GetMethod(string methodName)
        {
            if (!Enum.TryParse(methodName, out FfiMethod method))
            {
                throw new InvalidCastException($"Failed to parse {methodName}");
            }
            
            Logger.Verbose("Parsed method: {0}->{1}", 
                methodName,
                method);
            return method;

        }

        #endregion

        #region IDisposable members

        ~FilterListManager()
        {
            Dispose(false);
        }
        
        private void ReleaseManagedResources()
        {
            m_FLMHandle = IntPtr.Zero;
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
