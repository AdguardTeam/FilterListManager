using System;
using System.Collections.Generic;
using System.IO;
using AdGuard.Utils.Base.DllProvider;
using AdGuard.Utils.Base.DriverInstaller;

namespace AdGuard.FilterListManager.Utils
{
    /// <summary>
    /// Class for defining the required native dlls
    /// </summary>
    public class FilterManagerDllProvider : LibsDllProviderBase
    {
        private const string WIN32_DLL_NAME = @"x86\" + Constants.RUST_DLL_NAME;
        private const string WIN64_DLL_NAME = @"x64\" + Constants.RUST_DLL_NAME;
        private const string ARM64_DLL_NAME = @"arm64\" + Constants.RUST_DLL_NAME;

        private static readonly Dictionary<ArchitectureLocal, string> DLL_PATHS_MAP =
            new Dictionary<ArchitectureLocal, string>
            {
                {
                    ArchitectureLocal.X86,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            WIN32_DLL_NAME)
                },
                {
                    ArchitectureLocal.X64,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            WIN64_DLL_NAME)
                },
                {
                    ArchitectureLocal.Arm,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            WIN32_DLL_NAME)
                },
                {
                    ArchitectureLocal.Arm64,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            ARM64_DLL_NAME)
                }
            };

        /// <summary>
        /// Native libs DLL provider
        /// </summary>
        public FilterManagerDllProvider() : base(DLL_PATHS_MAP)
        {
        }

        private static readonly Lazy<FilterManagerDllProvider> LAZY =
            new Lazy<FilterManagerDllProvider>(() => new FilterManagerDllProvider());

        #region Singleton

        /// <summary>
        /// Gets a singleton instance of <see cref="FilterManagerDllProvider"/> object
        /// </summary>
        public static ILibsDllProvider Instance
        {
            get { return LAZY.Value; }
        }

        #endregion
    }
}