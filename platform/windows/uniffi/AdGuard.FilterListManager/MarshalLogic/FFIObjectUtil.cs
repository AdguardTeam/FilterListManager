using System;
using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Utils method for FFI objects
    /// </summary>
    public static class FfiObjectUtil
    {
        /// <summary>
        /// Disposes all the sub-objects of the specified ones.
        /// </summary>
        /// <param name="list">The list.</param>
        public static void DisposeAll(params object[] list)
        {
            foreach (object obj in list)
            {
                Dispose(obj);
            }
        }

        /// <summary>
        /// Dispose is implemented by recursive type inspection at runtime. This is because
        /// generating correct Dispose calls for recursive complex types, e.g. List(List(int))
        /// is quite cumbersome.
        /// </summary>
        /// <param name="obj">The object.</param>
        private static void Dispose(dynamic obj)
        {
            if (obj == null)
            {
                return;
            }

            if (obj is IDisposable disposable)
            {
                disposable.Dispose();
                return;
            }

            var type = obj.GetType();
            if (type != null)
            {
                if (type.IsGenericType)
                {
                    if (type.GetGenericTypeDefinition().IsAssignableFrom(typeof(List<>)))
                    {
                        foreach (var value in obj)
                        {
                            Dispose(value);
                        }
                    }
                    else if (type.GetGenericTypeDefinition().IsAssignableFrom(typeof(Dictionary<,>)))
                    {
                        foreach (var value in obj.Values)
                        {
                            Dispose(value);
                        }
                    }
                }
            }
        }
    }
}