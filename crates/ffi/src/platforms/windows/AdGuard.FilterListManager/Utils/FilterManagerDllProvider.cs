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
        private const string Win32DllName = @"x86\" + Constants.FLM_DLL_NAME;
        private const string Win64DllName = @"x64\" + Constants.FLM_DLL_NAME;
        private const string Arm64DllName = @"arm64\" + Constants.FLM_DLL_NAME;

        private static readonly Dictionary<ArchitectureLocal, string> DllPathsMap =
            new Dictionary<ArchitectureLocal, string>
            {
                {
                    ArchitectureLocal.X86,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            Win32DllName)
                },
                {
                    ArchitectureLocal.X64,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            Win64DllName)
                },
                {
                    ArchitectureLocal.Arm,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            Win32DllName)
                },
                {
                    ArchitectureLocal.Arm64,
                        Path.Combine(
                            AppDomain.CurrentDomain.BaseDirectory,
                            Arm64DllName)
                }
            };

        /// <summary>
        /// Native libs DLL provider
        /// </summary>
        public FilterManagerDllProvider() : base(DllPathsMap)
        {
        }

        private static readonly Lazy<FilterManagerDllProvider> Lazy =
            new Lazy<FilterManagerDllProvider>(() => new FilterManagerDllProvider());

        #region Singleton

        /// <summary>
        /// Gets a singleton instance of <see cref="FilterManagerDllProvider"/> object
        /// </summary>
        public static ILibsDllProvider Instance
        {
            get { return Lazy.Value; }
        }

        #endregion
    }
}