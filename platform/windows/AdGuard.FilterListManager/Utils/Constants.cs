using AdGuard.FilterListManager.MarshalLogic;

namespace AdGuard.FilterListManager.Utils
{
    /// <summary>
    /// Main lib constants
    /// </summary>
    public class Constants
    {
        /// <summary>
        /// The Rust DLL file name
        /// </summary>
        public const string RUST_DLL_NAME = RUST_DLL_IMPORT_NAME + ".dll";

        /// <summary>
        /// The rust DLL file name for import calls
        /// </summary>
        public const string RUST_DLL_IMPORT_NAME = "AdGuardFLM";

        /// <summary>
        /// Gets the constants structure.
        /// </summary>
        public static FilterListManagerConstants GetConstantsStructure()
        {
            return FfiConverterTypeFilterListManagerConstants.Instance.Lift(
                UniffiHelpers.RustCall((ref RustCallStatus status) =>
                    UniFfiLib.uniffi_filter_list_manager_ffi_fn_func_make_constants_structure(ref status)
                ));
        }

        /// <summary>
        /// Gets the default configuration.
        /// </summary>
        /// <returns></returns>
        public static Configuration GetDefaultConfiguration()
        {
            return FfiConverterTypeConfiguration.Instance.Lift(
                UniffiHelpers.RustCall((ref RustCallStatus status) =>
                    UniFfiLib.uniffi_filter_list_manager_ffi_fn_func_make_default_configuration(ref status)
                ));
        }
    }
}
