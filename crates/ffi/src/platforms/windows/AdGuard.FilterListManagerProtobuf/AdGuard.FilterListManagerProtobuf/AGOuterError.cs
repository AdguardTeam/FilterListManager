using AGErrorProtobuf = FilterListManager.AGOuterError;
using System;

namespace AdGuard.FilterListManagerProtobuf
{
    public class AGOuterError: Exception
    {
        public AGErrorProtobuf.ErrorOneofCase ErrorKind;

        public AGOuterError(AGErrorProtobuf from)
            : base(from.Message)
        {
            ErrorKind = from.ErrorCase;
        }
    }
}
