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
            byte[] resultDataBytes = GetResultData(ProtobufInterop.flm_default_configuration_protobuf, out IntPtr _);
            Configuration configuration = Configuration.Parser.ParseFrom(resultDataBytes);
            return configuration;
        }

        internal static IntPtr InitFLM(Configuration configuration)
        {
            byte[] data = configuration.ToByteArray();
            IntPtr pData = IntPtr.Zero;
            try
            {
                pData = Marshal.AllocHGlobal(data.Length);
                Marshal.Copy(data, 0, pData, data.Length);
                byte [] _ = GetResultData(
                    () => ProtobufInterop.flm_init_protobuf(pData, (ulong)data.Length), out IntPtr pResultData);
                return pResultData;
            }
            finally
            {
                MarshalUtils.SafeFreeHGlobal(pData);
            }
        }

        internal static byte[] CallRust(IntPtr flmHandle, FFIMethod method, byte[] args)
        {
            IntPtr pArgs = IntPtr.Zero;
            try
            {
                pArgs = Marshal.AllocHGlobal(args.Length);
                Marshal.Copy(args, 0, pArgs, args.Length);
                byte[] resultDataBytes = GetResultData(
                    () => ProtobufInterop.flm_call_protobuf(flmHandle, method, pArgs, (ulong)args.Length), out IntPtr _);
                return resultDataBytes;
            }
            finally
            {
                MarshalUtils.SafeFreeHGlobal(pArgs);
            }
        }
        
        internal static void FreeFLMHandle(IntPtr handle)
        {
            ProtobufInterop.flm_free_handle(handle);
        }
        
        private static byte[] GetResultData(Func<IntPtr> nativeFunc, out IntPtr pResultData)
        {
            IntPtr pRustResponse = IntPtr.Zero;
            try
            {
                pRustResponse = nativeFunc();
                RustResponse rustResponse = MarshalUtils.PtrToStructure<RustResponse>(pRustResponse);
                uint resultDataLen = rustResponse.ResultDataLen.ToUInt32();
                byte[] resultDataBytes = new byte[resultDataLen];
                Marshal.Copy(rustResponse.ResultData, resultDataBytes, 0, (int)resultDataLen);
                if (!rustResponse.FfiError)
                {
                    pResultData = rustResponse.ResultData;
                    return resultDataBytes;
                }
                
                AGErrorProtobuf error = AGErrorProtobuf.Parser.ParseFrom(resultDataBytes);
                throw new AGOuterException(error);
            }
            finally
            {
                if (pRustResponse != IntPtr.Zero)
                {
                    ProtobufInterop.flm_free_response(pRustResponse);
                }
            }
        }
    }
}
