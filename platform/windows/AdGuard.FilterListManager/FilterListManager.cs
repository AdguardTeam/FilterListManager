using System.Collections.Generic;
using AdGuard.FilterListManager.MarshalLogic;

namespace AdGuard.FilterListManager
{
    /// <summary>
    /// Main <see cref="IFilterListManager"/> implementation.
    /// </summary>
    /// <seealso cref="FFIObject&lt;FilterListManagerSafeHandle&gt;" />
    /// <seealso cref="IFilterListManager" />
    public class FilterListManager : FFIObject<FilterListManagerSafeHandle>, IFilterListManager
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
                            FfiConverterTypeConfiguration.INSTANCE.Lower(configuration),
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
            return FfiConverterBoolean.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_change_locale(
                            GetHandle(),
                            FfiConverterString.INSTANCE.Lower(suggestedLocale),
                            ref status
                        )
                )
            );
        }
        
        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public long DeleteCustomFilterLists(List<long> ids)
        {
            return FfiConverterInt64.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_delete_custom_filter_lists(
                            GetHandle(),
                            FfiConverterSequenceInt64.INSTANCE.Lower(ids),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public long EnableFilterLists(List<long> ids, bool isEnabled)
        {
            return FfiConverterInt64.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_enable_filter_lists(
                            GetHandle(),
                            FfiConverterSequenceInt64.INSTANCE.Lower(ids),
                            FfiConverterBoolean.INSTANCE.Lower(isEnabled),
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
            return FfiConverterTypeFilterListMetadata.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_fetch_filter_list_metadata(
                            GetHandle(),
                            FfiConverterString.INSTANCE.Lower(url),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public UpdateResult ForceUpdateFiltersByIds(List<long> ids, int looseTimeout)
        {
            return FfiConverterOptionalTypeUpdateResult.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_force_update_filters_by_ids(
                            GetHandle(),
                            FfiConverterSequenceInt64.INSTANCE.Lower(ids),
                            FfiConverterInt32.INSTANCE.Lower(looseTimeout),
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
            return FfiConverterSequenceTypeActiveRulesInfo.INSTANCE.Lift(
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
            return FfiConverterSequenceTypeFilterGroup.INSTANCE.Lift(
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
            return FfiConverterSequenceTypeFilterTag.INSTANCE.Lift(
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
            return FfiConverterString.INSTANCE.Lift(
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
            return FfiConverterOptionalInt32.INSTANCE.Lift(
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
        public FullFilterList GetFullFilterListById(long id)
        {
            return FfiConverterOptionalTypeFullFilterList.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_full_filter_list_by_id(
                            GetHandle(),
                            FfiConverterInt64.INSTANCE.Lower(id),
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
        public StoredFilterMetadata GetStoredFiltersMetadataById(long id)
        {
            return FfiConverterOptionalTypeStoredFilterMetadata.Instance.Lift(
                UniffiHelpers.RustCallWithError(FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib
                            .uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_stored_filters_metadata_by_id(GetHandle(), FfiConverterInt64.INSTANCE.Lower(id), ref status)
                ));
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public List<FullFilterList> GetFullFilterLists()
        {
            return FfiConverterSequenceTypeFullFilterList.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_full_filter_lists(
                            GetHandle(),
                            ref status
                        )
                )
            );
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
            return FfiConverterTypeFullFilterList.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_custom_filter_from_string(
                            GetHandle(),
                            FfiConverterString.INSTANCE.Lower(downloadUrl),
                            FfiConverterInt64.INSTANCE.Lower(lastDownloadTime),
                            FfiConverterBoolean.INSTANCE.Lower(isEnabled),
                            FfiConverterBoolean.INSTANCE.Lower(isTrusted),
                            FfiConverterString.INSTANCE.Lower(filterBody),
                            FfiConverterOptionalString.INSTANCE.Lower(customTitle),
                            FfiConverterOptionalString.INSTANCE.Lower(customDescription),
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
            return FfiConverterTypeFullFilterList.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_custom_filter_list(
                            GetHandle(),
                            FfiConverterString.INSTANCE.Lower(downloadUrl),
                            FfiConverterBoolean.INSTANCE.Lower(isTrusted),
                            FfiConverterOptionalString.INSTANCE.Lower(title),
                            FfiConverterOptionalString.INSTANCE.Lower(description),
                            ref status
                        )
                )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public long InstallFilterLists(List<long> ids, bool isInstalled)
        {
            return FfiConverterInt64.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_filter_lists(
                            GetHandle(),
                            FfiConverterSequenceInt64.INSTANCE.Lower(ids),
                            FfiConverterBoolean.INSTANCE.Lower(isInstalled),
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
                        FfiConverterTypeFilterListRules.INSTANCE.Lower(rules),
                        ref status
                    )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public void SaveDisabledRules(long filterId, List<string> disabledRules)
        {
            UniffiHelpers.RustCallWithError(
                FfiConverterTypeAgOuterException.Instance,
                (ref RustCallStatus status) =>
                    UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_save_disabled_rules(
                        GetHandle(),
                        FfiConverterInt64.INSTANCE.Lower(filterId),
                        FfiConverterSequenceString.INSTANCE.Lower(disabledRules),
                        ref status
                    )
            );
        }

        /// <summary>
        ///<inheritdoc cref="IFilterListManager"/>
        /// </summary>
        public bool UpdateCustomFilterMetadata(long filterId, string title, bool isTrusted)
        {
            return FfiConverterBoolean.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_update_custom_filter_metadata(
                            GetHandle(),
                            FfiConverterInt64.INSTANCE.Lower(filterId),
                            FfiConverterString.INSTANCE.Lower(title),
                            FfiConverterBoolean.INSTANCE.Lower(isTrusted),
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
            return FfiConverterOptionalTypeUpdateResult.INSTANCE.Lift(
                UniffiHelpers.RustCallWithError(
                    FfiConverterTypeAgOuterException.Instance,
                    (ref RustCallStatus status) =>
                        UniFfiLib.uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_update_filters(
                            GetHandle(),
                            FfiConverterBoolean.INSTANCE.Lower(ignoreFiltersExpiration),
                            FfiConverterInt32.INSTANCE.Lower(looseTimeout),
                            FfiConverterBoolean.INSTANCE.Lower(ignoreFiltersStatus),
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
    }
}
