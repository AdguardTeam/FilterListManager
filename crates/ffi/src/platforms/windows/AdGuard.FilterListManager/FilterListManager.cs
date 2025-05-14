using System;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using AdGuard.FilterListManager.Api;
using AdGuard.FilterListManager.Api.Exceptions;
using AdGuard.FilterListManager.RustInterface;
using AdGuard.Utils.Base.Logging;
using AdGuard.Utils.Serializers;
using FilterListManager;
using Google.Protobuf;

namespace AdGuard.FilterListManager
{
    public class FilterListManager : IFilterListManager
    {
        // ReSharper disable once InconsistentNaming
        private IntPtr m_FLMHandle;
        private readonly ISerializer<byte[]> m_Serializer;

        /// <summary>
        /// Spawns a struct of Filter List Manager public constants 
        /// </summary>
        /// <returns></returns>
        public static FLMConstants SpawnDefaultConstants()
        {
            return ProtobufInterop.flm_get_constants();
        }

        public FilterListManager(ISerializer<byte[]> serializer)
        {
            m_Serializer = serializer;
        }

        #region IFilterListManager members

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void Init(Configuration configuration)
        {
            IntPtr FlmInteropFunc(IntPtr pHandle, FfiMethod ffiMethod, IntPtr pInputData, ulong inputDataLength) =>
                ProtobufInterop.flm_init_protobuf(pInputData, inputDataLength);
            m_FLMHandle = CallRustHandle<AGOuterError>(configuration, FlmInteropFunc);
        }

        /// <summary>
        ///  header - crates/ffi/src/flm_native_interface.h
        ///  swift.file - crates/ffi/src/platforms/apple/flmctest/flmctest/main.swift
        ///  protobuf schema - crates/ffi/src/protobuf
        /// </summary>
        /// Spawns configuration for set up
        public Configuration SpawnDefaultConfiguration()
        {
            IntPtr FlmInteropFunc(IntPtr pHandle, FfiMethod ffiMethod, IntPtr pInputData, ulong inputDataLength) =>
                ProtobufInterop.flm_default_configuration_protobuf();
            EmptyRequest request = new EmptyRequest();
            return CallRustMessage<Configuration>(request, FlmInteropFunc);
        }

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
                CallRustMessage<InstallCustomFilterListResponse>(request);
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
            EnableFilterListsResponse response = CallRustMessage<EnableFilterListsResponse>(request);
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
                CallRustMessage<InstallFilterListsResponse>(request);
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
                CallRustMessage<DeleteCustomFilterListsResponse>(request);
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

            GetFullFilterListByIdResponse response = CallRustMessage<GetFullFilterListByIdResponse>(request);
            return response.FilterList;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<StoredFilterMetadata> GetStoredFiltersMetadata()
        {
            EmptyRequest request = new EmptyRequest();
            GetStoredFiltersMetadataResponse response =
                CallRustMessage<GetStoredFiltersMetadataResponse>(request);
            return response.FilterLists;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public StoredFilterMetadata GetStoredFilterMetadataById(int filterId)
        {
            GetStoredFilterMetadataByIdRequest request = new GetStoredFilterMetadataByIdRequest
            {
                Id = filterId
            };

            GetStoredFilterMetadataByIdResponse response =
                CallRustMessage<GetStoredFilterMetadataByIdResponse>(request);
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

            CallRustMessage<EmptyResponse>(request);
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
            CallRustMessage<EmptyResponse>(request);
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

            UpdateFiltersResponse response = CallRustMessage<UpdateFiltersResponse>(request);
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
                CallRustMessage<ForceUpdateFiltersByIdsResponse>(request);
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
                CallRustMessage<FetchFilterListMetadataResponse>(request);
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
                CallRustMessage<FetchFilterListMetadataWithBodyResponse>(request);
            return response.Metadata;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void LiftUpDatabase()
        {
            EmptyRequest request = new EmptyRequest();
            CallRustMessage<EmptyResponse>(request);
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<FilterTag> GetAllTags()
        {
            EmptyRequest request = new EmptyRequest();
            GetAllTagsResponse response = CallRustMessage<GetAllTagsResponse>(request);
            return response.Tags;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<FilterGroup> GetAllGroups()
        {
            EmptyRequest request = new EmptyRequest();
            GetAllGroupsResponse response = CallRustMessage<GetAllGroupsResponse>(request);
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

            ChangeLocaleResponse response = CallRustMessage<ChangeLocaleResponse>(request);
            return response.Success;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public PullMetadataResult PullMetadata()
        {
            EmptyRequest request = new EmptyRequest();
            PullMetadataResponse response = CallRustMessage<PullMetadataResponse>(request);
            return response.Result;
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

            UpdateCustomFilterMetadataResponse response = CallRustMessage<UpdateCustomFilterMetadataResponse>(request);
            return response.Success;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public string GetDatabasePath()
        {
            EmptyRequest request = new EmptyRequest();
            GetDatabasePathResponse response = CallRustMessage<GetDatabasePathResponse>(request);
            return response.Path;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public int GetDatabaseVersion()
        {
            EmptyRequest request = new EmptyRequest();
            GetDatabaseVersionResponse response = CallRustMessage<GetDatabaseVersionResponse>(request);
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

            InstallCustomFilterFromStringResponse response = CallRustMessage<InstallCustomFilterFromStringResponse>(request);
            return response.FilterList;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<ActiveRulesInfo> GetActiveRules()
        {
            EmptyRequest request = new EmptyRequest();
            GetActiveRulesResponse response = CallRustMessage<GetActiveRulesResponse>(request);
            return response.Rules;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<FilterListRulesRaw> GetFilterRulesAsStrings(IEnumerable<int> filterIds)
        {
            GetFilterRulesAsStringsRequest request = new GetFilterRulesAsStringsRequest();
            request.Ids.AddRange(filterIds);
            GetFilterRulesAsStringsResponse response = CallRustMessage<GetFilterRulesAsStringsResponse>(request);
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

            CallRustMessage<EmptyResponse>(request);
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<DisabledRulesRaw> GetDisabledRules(IEnumerable<int> filterIds)
        {
            GetDisabledRulesRequest request = new GetDisabledRulesRequest();
            request.Ids.AddRange(filterIds);
            GetDisabledRulesResponse response = CallRustMessage<GetDisabledRulesResponse>(request);
            return response.RulesRaw;
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public void SetProxyMode(string customProxyAddr, RawRequestProxyMode proxyMode)
        {
            SetProxyModeRequest request = new SetProxyModeRequest
            {
                CustomProxyAddr = customProxyAddr,
                Mode = proxyMode
            };

            CallRustMessage<EmptyResponse>(request);
        }

        /// <summary>
        /// <inheritdoc/>
        /// </summary>
        public IEnumerable<RulesCountByFilter> GetRulesCount(IEnumerable<int> filterIds)
        {
            GetRulesCountRequest request = new GetRulesCountRequest();
            request.Ids.AddRange(filterIds);
            GetRulesCountResponse response = CallRustMessage<GetRulesCountResponse>(request);
            return response.RulesCountByFilter;
        }

        #endregion


        #region Helpers

        private IntPtr CallRustHandle<TOutMessage>(
            IMessage inMessage,
            Func<IntPtr, FfiMethod, IntPtr, ulong, IntPtr> flmInteropFunc,
            [CallerMemberName] string ffiMethodName = null)
            where TOutMessage : IMessage, IAGOuterError
        {
            CallRust(
                m_FLMHandle,
                inMessage,
                out TOutMessage _,
                out IntPtr outHandle,
                flmInteropFunc,
                ffiMethodName);
            return outHandle;
        }

        private TOutMessage CallRustMessage<TOutMessage>(
            IMessage inMessage,
            Func<IntPtr, FfiMethod, IntPtr, ulong, IntPtr> flmInteropFunc,
            [CallerMemberName] string ffiMethodName = null)
            where TOutMessage : IMessage, IAGOuterError
        {
            CallRust(
                m_FLMHandle,
                inMessage,
                out TOutMessage outMessage,
                out IntPtr _,
                flmInteropFunc,
                ffiMethodName);
            return outMessage;
        }

        private TOutMessage CallRustMessage<TOutMessage>(
            IMessage inMessage,
            [CallerMemberName] string ffiMethodName = null)
            where TOutMessage : IMessage, IAGOuterError
        {
            CallRust(
                m_FLMHandle,
                inMessage,
                out TOutMessage outMessage,
                out IntPtr _,
                ProtobufInterop.flm_call_protobuf,
                ffiMethodName);
            return outMessage;
        }

        private void CallRust<TOutMessage>(
            IntPtr flmHandle,
            IMessage inMessage,
            out TOutMessage outMessage,
            out IntPtr outHandle,
            Func<IntPtr, FfiMethod, IntPtr, ulong, IntPtr> flmInteropFunc,
            [CallerMemberName] string ffiMethodName = null)
            where TOutMessage : IMessage, IAGOuterError
        {
            string errorTemplate = $"Cannot call RUST method \"{ffiMethodName}\"";
            FfiMethod ffiMethod = GetFfiMethod(ffiMethodName);
            if (flmHandle == IntPtr.Zero &&
                // only two methods below can be invoked correctly without set flmHandle before
                ffiMethod != FfiMethod.SpawnDefaultConfiguration &&
                ffiMethod != FfiMethod.Init)
            {
                string errorMessage = $"instance must be initialized with \"{nameof(Init)}\" before invocation";
                Logger.Error(errorTemplate);
                throw new FilterListManagerNotInitializedException(errorMessage);
            }

            try
            {
                outMessage = default;
                outHandle = default;
                byte[] args = inMessage?.ToByteArray() ?? new byte[0];
                RustResponseResult rustResponseResult = ProtobufBridge.CallRust(flmHandle, ffiMethod, args, flmInteropFunc);
                TOutMessage response = default;
                switch (rustResponseResult.Discriminant)
                {
                    case RustResponseType.RustBuffer:
                    {
                        response = m_Serializer.DeserializeObject<TOutMessage>(rustResponseResult.Buffer);
                        outMessage = response;
                        break;
                    }
                    case RustResponseType.FLMHandlePointer:
                    {
                        outHandle = rustResponseResult.HandlePointer;
                        break;
                    }
                    default:
                    {
                        string errorMessage = $"Invalid discriminant {rustResponseResult.Discriminant} was obtained while method {ffiMethodName} called";
                        throw new FilterListManagerInvalidDiscriminantException(errorMessage);
                    }
                }

                if (response?.Error != null)
                {
                    throw new AgOuterException(response.Error);
                }

                // special case is response represents AGOuterError explicitly.
                // This case is actual when ffiMethodName is looking for the RustResponseType.FLMHandlePointer
                // in the answer
                if (response is AGOuterError responseAgOuterError)
                {
                    throw new AgOuterException(responseAgOuterError);
                }
            }
            catch (Exception ex)
            {
                throw new FilterListManagerCommonException(errorTemplate, ex);
            }
        }

        private FfiMethod GetFfiMethod(string methodName)
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
            ProtobufInterop.flm_free_handle(m_FLMHandle);
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
