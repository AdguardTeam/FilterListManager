using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    internal interface ICallStatusErrorHandler<out T> where T : Exception
    {
        T Lift(RustBuffer errorBuf);
    }
}