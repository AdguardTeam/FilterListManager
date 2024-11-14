using System;
using System.Runtime.InteropServices;
using AdGuard.Utils.Base.Interop;
using FilterListManager;
using Google.Protobuf;
using AGErrorProtobuf = FilterListManager.AGOuterError;

namespace AdGuard.FilterListManagerProtobuf.RustInterface
{
    static class ProtobufBridge
    {
        internal static Configuration MakeDefaultConfiguration()
        {
            IntPtr responsePtr = ProtobufInterop.flm_default_configuration_protobuf();
            RustResponse response = MarshalUtils.PtrToStructure<RustResponse>(responsePtr);
            byte[] byteData = new byte[response.ResultDataLen];
            // NEEDS COPY? Or unsafe?
            Marshal.Copy(response.ResultData, byteData, 0, (int) response.ResultDataLen);
            ProtobufInterop.flm_free_response(responsePtr);
            if (response.FfiError)
            {
                AGErrorProtobuf error = AGErrorProtobuf.Parser.ParseFrom(byteData);
                throw new AGOuterException(error);
            }

            return Configuration.Parser.ParseFrom(byteData);
        }

        internal static IntPtr InitFLM(Configuration configuration)
        {
            byte[] data = configuration.ToByteArray();
            IntPtr pData = IntPtr.Zero;
            try
            {
                pData = Marshal.AllocHGlobal(data.Length);
                Marshal.Copy(data, 0, pData, data.Length);
                return ProtobufInterop.flm_init_protobuf(pData, (ulong)data.Length);
            }
            finally
            {
                MarshalUtils.SafeFreeHGlobal(pData);
            }
        }

        internal static byte[] CallRust(IntPtr flmHandle, FFIMethod method, byte[] args)
        {
            IntPtr pData = IntPtr.Zero;
            IntPtr pResponse = IntPtr.Zero;
            try
            {
                pData = Marshal.AllocHGlobal(args.Length);
                Marshal.Copy(args, 0, pData, args.Length);
                pResponse = ProtobufInterop.flm_call_protobuf(flmHandle, method, pData, (ulong)args.Length);
                RustResponse response = MarshalUtils.PtrToStructure<RustResponse>(pResponse);
                byte[] data = new byte[response.ResultDataLen];
                Marshal.Copy(pResponse, data, 0, (int) response.ResultDataLen);
                if (!response.FfiError)
                {
                    return data;
                }
                
                AGErrorProtobuf error = AGErrorProtobuf.Parser.ParseFrom(data);
                throw new AGOuterException(error);
            }
            finally
            {
                if (pResponse != IntPtr.Zero)
                {
                    ProtobufInterop.flm_free_response(pResponse);
                }
                
                MarshalUtils.SafeFreeHGlobal(pData);
            }
        }

        internal static void FreeFLMHandle(IntPtr handle)
        {
            ProtobufInterop.flm_free_handle(handle);
        }
    }
}
