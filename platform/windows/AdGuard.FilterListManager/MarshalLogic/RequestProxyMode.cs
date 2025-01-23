namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Possible proxy settings for the FLM
    /// </summary>
    public class RequestProxyMode
    {
        /// <summary>
        /// Use system proxy settings
        /// </summary>
        /// <seealso cref="RequestProxyMode" />
        public class UseSystemProxy : RequestProxyMode { }

        /// <summary>
        /// Don't use any proxy
        /// </summary>
        /// <seealso cref="RequestProxyMode" />
        public class NoProxy : RequestProxyMode { }

        /// <summary>
        /// Use custom proxy
        /// </summary>
        /// <seealso cref="RequestProxyMode" />
        public class UseCustomProxy : RequestProxyMode
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="UseCustomProxy"/> class.
            /// </summary>
            /// <param name="address">The address.</param>
            public UseCustomProxy(string address)
            {
                Address = address;
            }

            /// <summary>
            /// Gets the proxy address.
            /// </summary>
            public string Address { get;  }
        }
    }
}
