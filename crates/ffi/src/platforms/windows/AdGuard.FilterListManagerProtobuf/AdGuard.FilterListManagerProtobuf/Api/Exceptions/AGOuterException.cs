using AGOuterErrorProtobuf = FilterListManager.AGOuterError;

namespace AdGuard.FilterListManagerProtobuf.Api.Exceptions
{
    /// <summary>
    /// Main general exception, based on
    /// ../filter-list-manager/browse/crates/ffi/src/protobuf/outer_error.proto 
    /// </summary>
    public class AgOuterException : FilterListManagerBaseException
    {
        /// <summary>
        /// Kind of AgOuterException set by FLM itself
        /// </summary>
        public AGOuterErrorProtobuf.ErrorOneofCase ErrorKind { get; private set; }

        /// <summary>
        /// Creates new instance of <see cref="AgOuterException"/>
        /// based on protobuf-generated <see cref="from"/>
        /// </summary>
        public AgOuterException(AGOuterErrorProtobuf from)
            : base(from.Message)
        {
            ErrorKind = from.ErrorCase;
        }
    }
}
