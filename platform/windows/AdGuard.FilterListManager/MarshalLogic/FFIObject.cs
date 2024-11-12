using System;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Base holder class for disposable handle
    /// </summary>
    /// <typeparam name="THandle">The type of the handle.</typeparam>
    /// <seealso cref="IDisposable" />
    public abstract class FfiObject<THandle> : IDisposable
        where THandle : FfiSafeHandle
    {
        private readonly THandle m_handle;

        protected FfiObject(THandle handle)
        {
            m_handle = handle;
        }

        /// <summary>
        /// Gets the handle.
        /// </summary>
        public THandle GetHandle()
        {
            return m_handle;
        }

        /// <summary>
        /// <inheritdoc cref="IDisposable"/>
        /// </summary>
        public void Dispose()
        {
            m_handle.Dispose();
        }
    }
}