using System.Collections.Generic;
using AdGuard.FilterListManager.MarshalLogic;

namespace AdGuard.FilterListManager
{
    /// <summary>
    /// Main <see cref="IFilterListManager"/> implementation.
    /// </summary>
    /// <seealso cref="FfiObject{THandle}" />
    /// <seealso cref="IFilterListManager" />
    public class FilterListManager : FfiObject<FilterListManagerSafeHandle>, IFil-terListManager
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListManager"/> class.
        /// </summary>
        /// <param name="pointer">The pointer.</param>
        public FilterListManager(FilterListManagerSafeHandle pointer)
            : base(pointer) { }

        /// <summary>
        /// Initializes a new instance of the <see cref="FilterListManager"/> class.
        /// </summary>
        /// <param name="configuration">The configuration.</param>
        public FilterListManager(Configuration configuration)
            : this(
                UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_constructor_filterlistmanager_new(
                            FfiConverterTypeConfiguration.Instance.Lower(configuration),
                            ref status
                        )
                )
            )
        { }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public bool ChangeLocale(string suggestedLocale)
        {
            return FfiConverterBoolean.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_change_locale(
                            GetHandle(),
                            FfiConverterString.Instance.Lower(suggestedLocale),
                            ref status
                        )
                )
            );
        }
        
        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public long DeleteCustomFilterLists(List<int> ids)
        {
            return FfiConverterInt64.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_delete_custom_filter_lists(
                            GetHandle(),
                            FfiConverterSequenceInt32.Instance.Lower(ids),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public long EnableFilterLists(List<int> ids, bool isEnabled)
        {
            return FfiConverterInt64.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_enable_filter_lists(
                            GetHandle(),
                            FfiConverterSequenceInt32.Instance.Lower(ids),
                            FfiConverterBoolean.Instance.Lower(isEnabled),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public FilterListMetadata FetchFilterListMetadata(string url)
        {
            return FfiConverterTypeFilterListMetadata.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_fetch_filter_list_metadata(
                            GetHandle(),
                            FfiConverterString.Instance.Lower(url),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public UpdateResult ForceUpdateFiltersByIds(List<int> ids, int looseTimeout)
        {
            return FfiConverterOptionalTypeUpdateResult.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_force_update_filters_by_ids(
                            GetHandle(),
                            FfiConverterSequenceInt32.Instance.Lower(ids),
                            FfiConverterInt32.Instance.Lower(looseTimeout),
                            ref status
                        )
                )
            );
        }


        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public List<ActiveRulesInfo> GetActiveRules()
        {
            return FfiConverterSequenceTypeActiveRulesInfo.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_active_rules(
                            GetHandle(),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public List<FilterGroup> GetAllGroups()
        {
            return FfiConverterSequenceTypeFilterGroup.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_all_groups(
                            GetHandle(),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public List<FilterTag> GetAllTags()
        {
            return FfiConverterSequenceTypeFilterTag.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_all_tags(
                            GetHandle(),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public string GetDatabasePath()
        {
            return FfiConverterString.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_database_path(
                            GetHandle(),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public int? GetDatabaseVersion()
        {
            return FfiConverterOptionalInt32.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_database_version(
                            GetHandle(),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public List<DisabledRulesRaw> GetDisabledRules(List<int> ids)
        {
            return FfiConverterSequenceTypeDisabledRulesRaw.Instance.Lift(
                UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_disabled_rules(
                            GetHandle(), FfiConverterSequenceInt32.Instance.Lower(ids), ref status)
                ));
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public List<FilterListRulesRaw> GetFilterRulesAsStrings(List<int> ids)
        {
            return FfiConverterSequenceTypeFilterListRulesRaw.Instance.Lift(
                UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib
                            .uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_filter_rules_as_strings(
                                GetHandle(), FfiConverterSequenceInt32.Instance.Lower(ids), ref status)
                ));
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public FullFilterList GetFullFilterListById(int id)
        {
            return FfiConverterOptionalTypeFullFilterList.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_full_filter_list_by_id(
                            GetHandle(),
                            FfiConverterInt32.Instance.Lower(id),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public List<StoredFilterMetadata> GetStoredFiltersMetadata()
        {
            return FfiConverterSequenceTypeStoredFilterMetadata.Instance.Lift(
                UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib
                            .uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_stored_filters_metadata(
                                GetHandle(), ref status)
                ));
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public StoredFilterMetadata GetStoredFiltersMetadataById(int id)
        {
            return FfiConverterOptionalTypeStoredFilterMetadata.Instance.Lift(
                UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib
                            .uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_stored_filters_metadata_by_id(GetHandle(), FfiConverterInt32.Instance.Lower(id), ref status)
                ));
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public FullFilterList InstallCustomFilterFromString(
            string downloadUrl,
            long lastDownloadTime,
            bool isEnabled,
            bool isTrusted,
            string filterBody,
            string customTitle,
            string customDescription
        )
        {
            return FfiConverterTypeFullFilterList.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_custom_filter_from_string(
                            GetHandle(),
                            FfiConverterString.Instance.Lower(downloadUrl),
                            FfiConverterInt64.Instance.Lower(lastDownloadTime),
                            FfiConverterBoolean.Instance.Lower(isEnabled),
                            FfiConverterBoolean.Instance.Lower(isTrusted),
                            FfiConverterString.Instance.Lower(filterBody),
                            FfiConverterOptionalString.Instance.Lower(customTitle),
                            FfiConverterOptionalString.Instance.Lower(customDescription),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public FullFilterList InstallCustomFilterList(
            string downloadUrl,
            bool isTrusted,
            string title,
            string description
        )
        {
            return FfiConverterTypeFullFilterList.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_custom_filter_list(
                            GetHandle(),
                            FfiConverterString.Instance.Lower(downloadUrl),
                            FfiConverterBoolean.Instance.Lower(isTrusted),
                            FfiConverterOptionalString.Instance.Lower(title),
                            FfiConverterOptionalString.Instance.Lower(description),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public long InstallFilterLists(List<int> ids, bool isInstalled)
        {
            return FfiConverterInt64.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_filter_lists(
                            GetHandle(),
                            FfiConverterSequenceInt32.Instance.Lower(ids),
                            FfiConverterBoolean.Instance.Lower(isInstalled),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public void PullMetadata()
        {
            UniffiHelpers.RustCallWithError(
                FfiConverterTypeAgOuterException.Instance,
                (ref RustCallStatus status) =>
                    UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_pull_metadata(
                        GetHandle(),
                        ref status
                    )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public void SaveCustomFilterRules(FilterListRules rules)
        {
            UniffiHelpers.RustCallWithError(
                FfiConverterTypeAgOuterException.Instance,
                (ref RustCallStatus status) =>
                    UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_save_custom_filter_rules(
                        GetHandle(),
                        FfiConverterTypeFilterListRules.Instance.Lower(rules),
                        ref status
                    )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public void SaveDisabledRules(int filterId, List<string> disabledRules)
        {
            UniffiHelpers.RustCallWithError(
                FfiConverterTypeAgOuterException.Instance,
                (ref RustCallStatus status) =>
                    UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_save_disabled_rules(
                        GetHandle(),
                        FfiConverterInt32.Instance.Lower(filterId),
                        FfiConverterSequenceString.Instance.Lower(disabledRules),
                        ref status
                    )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public void SaveRulesToFileBlob(int filterId, string filePath)
        {
            UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance, (ref RustCallStatus status) =>
                UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_save_rules_to_file_blob(
                    GetHandle(), FfiConverterInt32.Instance.Lower(filterId),
                    FfiConverterString.Instance.Lower(filePath), ref status)
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public bool UpdateCustomFilterMetadata(int filterId, string title, bool isTrusted)
        {
            return FfiConverterBoolean.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_update_custom_filter_metadata(
                            GetHandle(),
                            FfiConverterInt32.Instance.Lower(filterId),
                            FfiConverterString.Instance.Lower(title),
                            FfiConverterBoolean.Instance.Lower(isTrusted),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public UpdateResult UpdateFilters(
            bool ignoreFiltersExpiration,
            int looseTimeout,
            bool ignoreFiltersStatus
        )
        {
            return FfiConverterOptionalTypeUpdateResult.Instance.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_update_filters(
                            GetHandle(),
                            FfiConverterBoolean.Instance.Lower(ignoreFiltersExpiration),
                            FfiConverterInt32.Instance.Lower(looseTimeout),
                            FfiConverterBoolean.Instance.Lower(ignoreFiltersStatus),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public void LiftUpDatabase()
        {
            UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance, (ref RustCallStatus status) =>
                UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_lift_up_database(
                    GetHandle(),
                    ref status)
            );
        }

        /// <exception cref="AgOuterException"></exception>
        public void SetProxyMode(RequestProxyMode requestProxyMode)
        {
            UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance, (ref RustCallStatus status) =>
                UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_set_proxy_mode(GetHandle(),
                    FfiConverterTypeRequestProxyMode.Instance.Lower(requestProxyMode), ref status)
            );
        }
    }
}
