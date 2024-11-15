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
            MarshalUtils.ag_buffer resultDataAgBuffer = GetNativeResult(
                ProtobufInterop.flm_default_configuration_protobuf);
            byte[] resultDataBytes = new byte[resultDataAgBuffer.size];
            Marshal.Copy(resultDataAgBuffer.data, resultDataBytes, 0, (int) resultDataAgBuffer.size);
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
                MarshalUtils.ag_buffer resultDataAgBuffer = GetNativeResult(
                    () => ProtobufInterop.flm_init_protobuf(pData, (ulong)data.Length));
                return resultDataAgBuffer.data;
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
                MarshalUtils.ag_buffer resultDataAgBuffer = GetNativeResult(
                    () => ProtobufInterop.flm_call_protobuf(flmHandle, method, pArgs, (ulong)args.Length));
                byte[] resultDataBytes = new byte[resultDataAgBuffer.size];
                Marshal.Copy(resultDataAgBuffer.data, resultDataBytes, 0, (int) resultDataAgBuffer.size);
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
        
        private static MarshalUtils.ag_buffer GetNativeResult(Func<IntPtr> nativeFunc)
        {
            IntPtr pRustResponse = IntPtr.Zero;
            try
            {
                pRustResponse = nativeFunc();
                RustResponse rustResponse = MarshalUtils.PtrToStructure<RustResponse>(pRustResponse);
                uint resultDataLen = rustResponse.ResultDataLen.ToUInt32();
                if (!rustResponse.FfiError)
                {
                    MarshalUtils.ag_buffer resultDataAgBuffer = new MarshalUtils.ag_buffer
                    {
                        data = rustResponse.ResultData,
                        size = resultDataLen
                    };
                    return resultDataAgBuffer;
                }
                
                byte[] resultDataBytes = new byte[resultDataLen];
                Marshal.Copy(rustResponse.ResultData, resultDataBytes, 0, (int)resultDataLen);
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
