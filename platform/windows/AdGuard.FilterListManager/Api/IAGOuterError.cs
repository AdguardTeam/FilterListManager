using FilterListManager;

namespace AdGuard.FilterListManager.Api
{
    /// <summary>
    /// Interface for proto-object contained <see cref="Error"/>
    /// </summary>
    // ReSharper disable once InconsistentNaming
    public interface IAGOuterError
    {
        /// <summary>
        /// Error
        /// </summary>
        AGOuterError Error { get; set; }
    }
}