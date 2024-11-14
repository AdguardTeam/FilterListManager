using AGOuterErrorProtobuf = FilterListManager.AGOuterError;
using System;

namespace AdGuard.FilterListManagerProtobuf
{
    public class AGOuterException: Exception
    {
        public AGOuterErrorProtobuf.ErrorOneofCase ErrorKind;

        public AGOuterException(AGOuterErrorProtobuf from)
            : base(from.Message)
        {
            ErrorKind = from.ErrorCase;
        }
    }
}
