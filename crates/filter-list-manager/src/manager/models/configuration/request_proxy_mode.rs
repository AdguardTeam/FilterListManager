/// Proxy mode of operation for requests
pub enum RequestProxyMode {
    /// System proxy will be used
    UseSystemProxy,
    /// All proxies disabled
    NoProxy,
    /// Use custom proxy
    UseCustomProxy { addr: String },
}
