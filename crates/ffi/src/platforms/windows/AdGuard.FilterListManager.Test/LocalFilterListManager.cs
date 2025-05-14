using System;
using AdGuard.FilterListManager.Api.Exceptions;
using AdGuard.FilterListManager.RustInterface;
using AdGuard.Utils.Serializers;
using FilterListManager;
using Google.Protobuf;

namespace AdGuard.FilterListManager.Test
{
    public class LocalFilterListManager : FilterListManager
    {
        public LocalFilterListManager(ISerializer<byte[]> serializer) : base(serializer)
        {
        }

        protected override void CallRust<TOutMessage>(IntPtr flmHandle, IMessage inMessage,
            out TOutMessage outMessage, out IntPtr outHandle, Func<IntPtr, FfiMethod, IntPtr, ulong, IntPtr> flmInteropFunc,
            string ffiMethodName = null)
        {
            try
            {
                base.CallRust(flmHandle, inMessage, out outMessage, out outHandle, flmInteropFunc, ffiMethodName);
            }
            catch (AgOuterException ex)
            {
                if (ex.ErrorKind != AGOuterError.ErrorOneofCase.HttpClientNetworkError)
                {
                    throw;
                }
            }

            outMessage = default;
            outHandle = default;
        }
    }
}