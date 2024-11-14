using System;
using FilterListManager;
using AdGuard.FilterListManagerProtobuf.RustInterface;
using Google.Protobuf;
using System.Collections.Generic;
using AdGuard.Utils.Serializers;

namespace AdGuard.FilterListManagerProtobuf
{
    public class FilterListManager : IFilterListManager
    {
        protected IntPtr FLMHandle;
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
            FLMHandle = ProtobufBridge.InitFLM(configuration);
            m_Serializer = serializer;
        }

        protected TMessage CallRust<TMessage>(FFIMethod method, IMessage message) 
            where TMessage : IMessage, IAGOuterError
        {
            byte[] args = message.ToByteArray();
            byte[] data = ProtobufBridge.CallRust(FLMHandle, method, args);
            TMessage response = m_Serializer.DeserializeObject<TMessage>(data);
            if (response.Error != null)
            {
                throw new AGOuterException(response.Error);
            }

            return response;
        }
        
        #region IFilterListManager members

        public FullFilterList InstallCustomFilterList(
            string downloadUrl,
            bool isTrusted,
            string title, /* Nullable */
            string description /* Nullable */)
        {
            InstallCustomFilterListRequest request = new InstallCustomFilterListRequest
            {
                DownloadUrl = downloadUrl,
                IsTrusted = isTrusted,
                Title = title,
                Description = description
            };

            InstallCustomFilterFromStringResponse response = 
                CallRust<InstallCustomFilterFromStringResponse>(
                    FFIMethod.InstallCustomFilterList, 
                    request);
            return response.FilterList;
        }

        public long EnableFilterLists(IEnumerable<long> ids, bool isEnabled) 
        {
            EnableFilterListsRequest request = new EnableFilterListsRequest
            {
                IsEnabled = isEnabled
            };

            request.Ids.AddRange(ids);
            EnableFilterListsResponse response = CallRust<EnableFilterListsResponse>(FFIMethod.EnableFilterLists, request);
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
                CallRust<InstallFilterListsResponse>(FFIMethod.EnableFilterLists, request);
            return response.Count;
        }

        public long DeleteCustomFilterLists(IEnumerable<long> ids)
        {
            DeleteCustomFilterListsRequest request = new DeleteCustomFilterListsRequest();
            request.Ids.AddRange(ids);
            DeleteCustomFilterListsResponse response = 
                CallRust<DeleteCustomFilterListsResponse>(FFIMethod.EnableFilterLists, request);
            return response.Count;
        }

        public FullFilterList GetFullFilterListById(long filterId)
        {
            GetFullFilterListByIdRequest request = new GetFullFilterListByIdRequest
            {
                Id = filterId
            };

            GetFullFilterListByIdResponse response = CallRust<GetFullFilterListByIdResponse>(FFIMethod.GetFullFilterListById, request);
            return response.FilterList;
        }

        public IEnumerable<StoredFilterMetadata> GetStoredFiltersMetadata()
        {
            EmptyRequest request = new EmptyRequest();
            GetStoredFiltersMetadataResponse response = 
                CallRust<GetStoredFiltersMetadataResponse>(FFIMethod.GetStoredFiltersMetadata, request);
            return response.FilterLists;
        }

        public StoredFilterMetadata GetStoredFilterMetadataById(long filterId)
        {
            GetStoredFiltersMetadataByIdRequest request = new GetStoredFiltersMetadataByIdRequest
            {
                Id = filterId
            };

            GetStoredFilterMetadataByIdResponse response = 
                CallRust<GetStoredFilterMetadataByIdResponse>(FFIMethod.GetStoredFilterMetadataById, request);
            return response.FilterList;
        }

        public void SaveCustomFilterRules(FilterListRules rules)
        {
            SaveCustomFilterRulesRequest request = new SaveCustomFilterRulesRequest
            {
                Rules = rules
            };

            CallRust<EmptyResponse>(FFIMethod.SaveCustomFilterRules, request);
        }

        public void SaveDisabledRules(long id, List<string> disabledRules)
        {
            SaveDisabledRulesRequest request = new SaveDisabledRulesRequest
            {
                FilterId = id
            };

            request.DisabledRules.AddRange(disabledRules);
            CallRust<EmptyResponse>(FFIMethod.SaveCustomFilterRules, request);
        }

        public UpdateResult UpdateFilters(
            bool ignoreFiltersExpiration,
            int looseTimeout,
            bool ignoreFilterStatus
        )
        {
            UpdateFiltersRequest message = new UpdateFiltersRequest
            {
                IgnoreFiltersExpiration = ignoreFiltersExpiration,
                LooseTimeout = looseTimeout,
                IgnoreFiltersStatus = ignoreFilterStatus
            };

            UpdateFiltersResponse response = 
                CallRust<UpdateFiltersResponse>(FFIMethod.UpdateFilters, message);
            return response.Result;
        }
        
        #endregion

        // TODO: Add methods in FFIMethod order 


        #region IDisposable members

        ~FilterListManager()
        {
            Dispose(false);
        }
        
        private void ReleaseManagedResources()
        {
            throw new NotImplementedException();
        }

        private void ReleaseUnmanagedResources()
        {
            ProtobufBridge.FreeFLMHandle(FLMHandle);
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
