using System;
using FilterListManager;
using AdGuard.FilterListManagerProtobuf.RustInterface;
using Google.Protobuf;
using System.Collections.Generic;

namespace AdGuard.FilterListManagerProtobuf
{

    public class FilterListManager
    {
        protected IntPtr FLMHandle;

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

        public FilterListManager(Configuration configuration)
        {
            FLMHandle = ProtobufBridge.InitFLM(configuration);
        }

        protected byte[] CallRust(FFIMethod method, IMessage message)
        {
            return ProtobufBridge.CallRust(FLMHandle, method, message.ToByteArray());
        }

        public FullFilterList InstallCustomFilterList(
            string downloadURL,
            bool isTrusted,
            string title, /* Nullable */
            string description /* Nullable */
        )
        {
            var request = new InstallCustomFilterListRequest
            {
                DownloadUrl = downloadURL,
                IsTrusted = isTrusted,
                Title = title,
                Description = description
            };

            var responseBytes = CallRust(FFIMethod.InstallCustomFilterList, request);
            var response = InstallCustomFilterFromStringResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.FilterList;
        }

        public long EnableFilterLists(IEnumerable<long> ids, bool isEnabled) {
            var request = new EnableFilterListsRequest
            {
                IsEnabled = isEnabled
            };

            request.Ids.AddRange(ids);

            var responseBytes = CallRust(FFIMethod.EnableFilterLists, request);
            var response = EnableFilterListsResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.Count;
        }

        public long InstallFilterLists(IEnumerable<long> ids, bool isInstalled)
        {
            var request = new InstallFilterListsRequest
            {
                IsInstalled = isInstalled
            };

            request.Ids.AddRange(ids);

            var responseBytes = CallRust(FFIMethod.EnableFilterLists, request);
            var response = InstallFilterListsResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.Count;
        }

        public long DeleteCustomFilterLists(IEnumerable<long> ids)
        {
            var request = new DeleteCustomFilterListsRequest();
            request.Ids.AddRange(ids);

            var responseBytes = CallRust(FFIMethod.EnableFilterLists, request);
            var response = DeleteCustomFilterListsResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.Count;
        }

        public IEnumerable<FullFilterList> GetFullFilterLists()
        {
            var request = new EmptyRequest();

            var responseBytes = CallRust(FFIMethod.EnableFilterLists, request);
            var response = GetFullFilterListsResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.FilterLists;
        }

        public FullFilterList GetFullFilterListById(long filterId)
        {
            var request = new GetFullFilterListByIdRequest
            {
                Id = filterId
            };

            var responseBytes = CallRust(FFIMethod.GetFullFilterListById, request);
            var response = GetFullFilterListByIdResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.FilterList;
        }

        public IEnumerable<StoredFilterMetadata> GetStoredFiltersMetadata()
        {
            var request = new EmptyRequest();

            var responseBytes = CallRust(FFIMethod.GetStoredFiltersMetadata, request);
            var response = GetStoredFiltersMetadataResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.FilterLists;
        }

        public StoredFilterMetadata GetStoredFilterMetadataById(long filterId)
        {
            var request = new GetStoredFiltersMetadataByIdRequest
            {
                Id = filterId
            };

            var responseBytes = CallRust(FFIMethod.GetStoredFilterMetadataById, request);
            var response = GetStoredFilterMetadataByIdResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.FilterList;
        }

        public void SaveCustomFilterRules(FilterListRules rules)
        {
            var request = new SaveCustomFilterRulesRequest
            {
                Rules = rules
            };

            var responseBytes = CallRust(FFIMethod.SaveCustomFilterRules, request);
            var response = EmptyResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }
        }

        public void SaveDisabledRules(long id, List<string> disabledRules)
        {
            var request = new SaveDisabledRulesRequest
            {
                FilterId = id
            };

            request.DisabledRules.AddRange(disabledRules);

            var responseBytes = CallRust(FFIMethod.SaveCustomFilterRules, request);
            var response = EmptyResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }
        }

        public UpdateResult UpdateFilters(
            bool ignoreFiltersExiration,
            int looseTimeout,
            bool ignoreFilterStatus
        )
        {
            var message = new UpdateFiltersRequest
            {
                IgnoreFiltersExpiration = ignoreFiltersExiration,
                LooseTimeout = looseTimeout,
                IgnoreFiltersStatus = ignoreFilterStatus
            };

            var responseBytes = CallRust(FFIMethod.UpdateFilters, message);
            var response = UpdateFiltersResponse.Parser.ParseFrom(responseBytes);

            if (response.Error != null)
            {
                throw new AGOuterError(response.Error);
            }

            return response.Result;
        }

        // TODO: Add methods in FFIMethod order 

        ~FilterListManager()
        {
            ProtobufBridge.FreeFLMHandle(FLMHandle);
        }
    }
}
