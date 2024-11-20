using System;
using AGOuterErrorProtobuf = FilterListManager.AGOuterError;

namespace AdGuard.FilterListManagerProtobuf.Api
{
    /// <summary>
    /// Main general exception, based on
    /// ../filter-list-manager/browse/crates/ffi/src/protobuf/outer_error.proto 
    /// </summary>
    public class AgOuterException: Exception
    {
        public AGOuterErrorProtobuf.ErrorOneofCase ErrorKind;

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
