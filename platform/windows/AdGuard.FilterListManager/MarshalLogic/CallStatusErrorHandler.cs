using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    interface CallStatusErrorHandler<E>
        where E : Exception
    {
        E Lift(RustBuffer error_buf);
    }
}