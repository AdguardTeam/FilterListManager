using System;
using System.Collections.Generic;
using System.Reflection;
using System.Runtime.InteropServices;
using AdGuard.FilterListManager.Utils;
// ReSharper disable IdentifierTypo
// ReSharper disable UnusedMember.Global

namespace AdGuard.FilterListManager.MarshalLogic
{
    internal static class UniFfiLib
    {
        static UniFfiLib()
        {
            UniffiCheckContractApiVersion();
            UniffiCheckApiChecksums();
        }
        
        #region Checksums
        
        static string GetDelegateMethodName(Delegate del)
        {
            MethodInfo method = del.Method;
            return method.Name;
        }

        private static readonly List<KeyValuePair<ushort, Func<ushort>>> API_CHECKSUM_MAP =
            new List<KeyValuePair<ushort, Func<ushort>>>
            {
                new KeyValuePair<ushort, Func<ushort>>(29147,
                    uniffi_filter_list_manager_ffi_checksum_func_make_constants_structure),
                new KeyValuePair<ushort, Func<ushort>>(58681,
                    uniffi_filter_list_manager_ffi_checksum_func_make_default_configuration),
                new KeyValuePair<ushort, Func<ushort>>(51409,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_change_locale),
                new KeyValuePair<ushort, Func<ushort>>(18921,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_delete_custom_filter_lists),
                new KeyValuePair<ushort, Func<ushort>>(5456,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_fetch_filter_list_metadata),
                new KeyValuePair<ushort, Func<ushort>>(34540,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_force_update_filters_by_ids),
                new KeyValuePair<ushort, Func<ushort>>(23881,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_all_groups),
                new KeyValuePair<ushort, Func<ushort>>(39010,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_all_tags),
                new KeyValuePair<ushort, Func<ushort>>(63965,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_database_path),
                new KeyValuePair<ushort, Func<ushort>>(16124,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_database_version),
                new KeyValuePair<ushort, Func<ushort>>(12308,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_full_filter_list_by_id),
                new KeyValuePair<ushort, Func<ushort>>(7447,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_full_filter_lists),
                new KeyValuePair<ushort, Func<ushort>>(17754,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_install_custom_filter_from_string),
                new KeyValuePair<ushort, Func<ushort>>(59518,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_install_custom_filter_list),
                new KeyValuePair<ushort, Func<ushort>>(43991,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_install_filter_lists),
                new KeyValuePair<ushort, Func<ushort>>(8269,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_pull_metadata),
                new KeyValuePair<ushort, Func<ushort>>(60344,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_save_custom_filter_rules),
                new KeyValuePair<ushort, Func<ushort>>(36828,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_save_disabled_rules),
                new KeyValuePair<ushort, Func<ushort>>(30898,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_update_custom_filter_metadata),
                new KeyValuePair<ushort, Func<ushort>>(2861,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_update_filters),
                new KeyValuePair<ushort, Func<ushort>>(14366,
                    uniffi_filter_list_manager_ffi_checksum_constructor_filterlistmanager_new),
                new KeyValuePair<ushort, Func<ushort>>(57711,
                    uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_active_rules)
            };

        static void UniffiCheckApiChecksums()
        {
            foreach (KeyValuePair<ushort, Func<ushort>> apiChecksum in API_CHECKSUM_MAP)
            {
                ushort checksum = apiChecksum.Value();
                if (checksum == apiChecksum.Key)
                {
                    continue;
                }

                string delegateName = GetDelegateMethodName(apiChecksum.Value);
                throw new UniffiContractChecksumException(
                    $"com.adguard.flm: uniffi bindings expected function `{delegateName}` checksum `{apiChecksum.Key}`, library returned `{checksum}`"
                );
            }
        }

        #endregion

        #region Imports
        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void uniffi_filter_list_manager_ffi_fn_free_filterlistmanager(
        IntPtr ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern FilterListManagerSafeHandle uniffi_filter_list_manager_ffi_fn_constructor_filterlistmanager_new(RustBuffer configuration, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern sbyte uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_change_locale(FilterListManagerSafeHandle ptr, RustBuffer suggestedLocale, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern long uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_delete_custom_filter_lists(FilterListManagerSafeHandle ptr, RustBuffer ids, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern long uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_enable_filter_lists(FilterListManagerSafeHandle ptr, RustBuffer ids, sbyte isEnabled, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_fetch_filter_list_metadata(FilterListManagerSafeHandle ptr, RustBuffer url, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_force_update_filters_by_ids(FilterListManagerSafeHandle ptr, RustBuffer ids, int looseTimeout, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_active_rules(FilterListManagerSafeHandle ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_all_groups(FilterListManagerSafeHandle ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_all_tags(FilterListManagerSafeHandle ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_database_path(FilterListManagerSafeHandle ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_database_version(FilterListManagerSafeHandle ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_full_filter_list_by_id(FilterListManagerSafeHandle ptr, long id, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_get_full_filter_lists(FilterListManagerSafeHandle ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_custom_filter_from_string(FilterListManagerSafeHandle ptr, RustBuffer downloadUrl, long lastDownloadTime, sbyte isEnabled, sbyte isTrusted, RustBuffer filterBody, RustBuffer customTitle, RustBuffer customDescription, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_custom_filter_list(FilterListManagerSafeHandle ptr, RustBuffer downloadUrl, sbyte isTrusted, RustBuffer title, RustBuffer description, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern long uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_install_filter_lists(FilterListManagerSafeHandle ptr, RustBuffer ids, sbyte isInstalled, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_pull_metadata(FilterListManagerSafeHandle ptr, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_save_custom_filter_rules(FilterListManagerSafeHandle ptr, RustBuffer rules, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_save_disabled_rules(FilterListManagerSafeHandle ptr, long filterId, RustBuffer disabledRules, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern sbyte uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_update_custom_filter_metadata(FilterListManagerSafeHandle ptr, long filterId, RustBuffer title, sbyte isTrusted, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_method_filterlistmanager_update_filters(FilterListManagerSafeHandle ptr, sbyte ignoreFiltersExpiration, int looseTimeout, sbyte ignoreFiltersStatus, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_func_make_constants_structure(ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer uniffi_filter_list_manager_ffi_fn_func_make_default_configuration(ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer ffi_filter_list_manager_ffi_rustbuffer_alloc(int size, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer ffi_filter_list_manager_ffi_rustbuffer_from_bytes(ForeignBytes bytes, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rustbuffer_free(RustBuffer buf, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer ffi_filter_list_manager_ffi_rustbuffer_reserve(RustBuffer buf, int additional, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_continuation_callback_set(IntPtr callback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_u8(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_u8(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_u8(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern byte ffi_filter_list_manager_ffi_rust_future_complete_u8(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_i8(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_i8(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_i8(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern sbyte ffi_filter_list_manager_ffi_rust_future_complete_i8(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_u16(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_u16(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_u16(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort ffi_filter_list_manager_ffi_rust_future_complete_u16(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_i16(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_i16(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_i16(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern short ffi_filter_list_manager_ffi_rust_future_complete_i16(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_u32(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_u32(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_u32(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern uint ffi_filter_list_manager_ffi_rust_future_complete_u32(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_i32(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_i32(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_i32(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern int ffi_filter_list_manager_ffi_rust_future_complete_i32(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_u64(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_u64(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_u64(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ulong ffi_filter_list_manager_ffi_rust_future_complete_u64(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_i64(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_i64(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_i64(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern long ffi_filter_list_manager_ffi_rust_future_complete_i64(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_f32(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_f32(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_f32(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern float ffi_filter_list_manager_ffi_rust_future_complete_f32(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_f64(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_f64(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_f64(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern double ffi_filter_list_manager_ffi_rust_future_complete_f64(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_pointer(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_pointer(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_pointer(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern SafeHandle ffi_filter_list_manager_ffi_rust_future_complete_pointer(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_rust_buffer(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_rust_buffer(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_rust_buffer(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern RustBuffer ffi_filter_list_manager_ffi_rust_future_complete_rust_buffer(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_poll_void(IntPtr handle, IntPtr uniffiCallback
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_cancel_void(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_free_void(IntPtr handle
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern void ffi_filter_list_manager_ffi_rust_future_complete_void(IntPtr handle, ref RustCallStatus uniffiOutErr
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_func_make_constants_structure(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_func_make_default_configuration(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_change_locale(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_delete_custom_filter_lists(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_enable_filter_lists(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_fetch_filter_list_metadata(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_force_update_filters_by_ids(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_active_rules(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_all_groups(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_all_tags(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_database_path(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_database_version(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_full_filter_list_by_id(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_get_full_filter_lists(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_install_custom_filter_from_string(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_install_custom_filter_list(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_install_filter_lists(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_pull_metadata(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_save_custom_filter_rules(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_save_disabled_rules(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_update_custom_filter_metadata(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_method_filterlistmanager_update_filters(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern ushort uniffi_filter_list_manager_ffi_checksum_constructor_filterlistmanager_new(
        );

        [DllImport(Constants.RUST_DLL_IMPORT_NAME)]
        public static extern uint ffi_filter_list_manager_ffi_uniffi_contract_version(
        );

        #endregion

        static void UniffiCheckContractApiVersion()
        {
            uint scaffoldingContractVersion =
                ffi_filter_list_manager_ffi_uniffi_contract_version();
            if (24 != scaffoldingContractVersion)
            {
                throw new UniffiContractVersionException(
                    $"com.adguard.flm: uniffi bindings expected version `24`, library returned `{scaffoldingContractVersion}`"
                );
            }
        }
    }
}