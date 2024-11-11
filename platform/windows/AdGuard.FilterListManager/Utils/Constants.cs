namespace AdGuard.FilterListManager.Utils
{
    /// <summary>
    /// Main lib constants
    /// </summary>
    internal class Constants
    {
        /// <summary>
        /// The Rust DLL file name
        /// </summary>
        public const string RUST_DLL_NAME = RUST_DLL_IMPORT_NAME + ".dll";

        /// <summary>
        /// The rust DLL file name for import calls
        /// </summary>
        public const string RUST_DLL_IMPORT_NAME = "AdGuardFLM";
    }
}
