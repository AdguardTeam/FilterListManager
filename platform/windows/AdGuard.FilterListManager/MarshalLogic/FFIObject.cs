using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    public abstract class FFIObject<THandle> : IDisposable
        where THandle : FFISafeHandle
    {
        private THandle handle;

        public FFIObject(THandle handle)
        {
            this.handle = handle;
        }

        public THandle GetHandle()
        {
            return handle;
        }

        public void Dispose()
        {
            handle.Dispose();
        }
    }
}