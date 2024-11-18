using System;
using System.Collections.Generic;
using System.IO;
using AdGuard.Utils.Base.DllProvider;
using AdGuard.Utils.Base.DriverInstaller;

namespace AdGuard.FilterListManagerProtobuf.Api
{
    /// <summary>
    /// Class for defining the required FLM libs dll
    /// </summary>
    /// <seealso cref="LibsDllProviderBase" />
    public class FlmDllProvider : LibsDllProviderBase
    {
        private static string m_Win32FlmDllName;
        private static string m_Win64FlmDllName;
        private static string m_Arm64FlmDllName;
        
        /// <summary>
        /// Sets the main FLM dll name
        /// </summary>
        /// <param name="flmDllName">FLM dll name</param>
        public static void SetFlmDllName(string flmDllName)
        {
            if (string.IsNullOrEmpty(flmDllName))
            {
                throw new ArgumentNullException(nameof(flmDllName), "Flm dll name must be specified");
            }

            m_Win32FlmDllName = $@"x86\{flmDllName}.dll";
            m_Win64FlmDllName = $@"x64\{flmDllName}.dll";
            m_Arm64FlmDllName = $@"Arm64\{flmDllName}.dll";

            #region FLM_DLL_PATHES_MAP

            m_FlmDllPathsMap =
                new Dictionary<ArchitectureLocal, string>
                {
                    {
                        ArchitectureLocal.X86,
                            Path.Combine(
                                AppDomain.CurrentDomain.BaseDirectory,
                                m_Win32FlmDllName)
                    },
                    {
                        ArchitectureLocal.X64,
                            Path.Combine(
                                AppDomain.CurrentDomain.BaseDirectory,
                                m_Win64FlmDllName)
                    },
                    {
                        ArchitectureLocal.Arm,
                            Path.Combine(
                                AppDomain.CurrentDomain.BaseDirectory,
                                m_Win32FlmDllName)
                    },
                    {
                        ArchitectureLocal.Arm64,
                            Path.Combine(
                                AppDomain.CurrentDomain.BaseDirectory,
                                m_Arm64FlmDllName)
                    }
                };

            #endregion
        }

        /// <summary>
        /// The main dll names were changed to "win32" versions for all kinds of architectures
        /// due to https://jira.adguard.com/browse/AG-17629
        /// </summary>
        private static Dictionary<ArchitectureLocal, string> m_FlmDllPathsMap;

        /// <summary>
        /// Initializes a new instance of the <see cref="FlmDllProvider"/> class.
        /// </summary>
        public FlmDllProvider() : base(m_FlmDllPathsMap)
        {
        }

        private static readonly Lazy<FlmDllProvider> Lazy =
            new Lazy<FlmDllProvider>(() => new FlmDllProvider());

        #region Singleton

        /// <summary>
        /// Gets a singleton instance of <see cref="FlmDllProvider"/> object
        /// </summary>
        public static ILibsDllProvider Instance => Lazy.Value;

        #endregion
    }
}
