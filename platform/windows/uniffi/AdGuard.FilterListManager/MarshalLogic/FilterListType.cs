namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Type of filter
    /// </summary>
    public enum FilterListType : int
    {
        /// <summary>
        /// The standard ad-blocking filter
        /// </summary>
        Standard,

        /// <summary>
        /// The DNS filter
        /// </summary>
        Dns
    }
}