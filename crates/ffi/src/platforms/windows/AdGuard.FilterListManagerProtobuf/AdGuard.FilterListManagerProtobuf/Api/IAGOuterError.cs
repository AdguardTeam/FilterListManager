using FilterListManager;

namespace AdGuard.FilterListManagerProtobuf.Api
{
    /// <summary>
    /// Interface for proto-object contained <see cref="Error"/>
    /// </summary>
    // ReSharper disable once InconsistentNaming
    public interface IAGOuterError
    {
        AGOuterError Error { get; set; }
    }
}