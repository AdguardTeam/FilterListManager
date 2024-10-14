using System;
using System.Runtime.InteropServices;
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
            RustResponse response = (RustResponse) Marshal.PtrToStructure(responsePtr, typeof(RustResponse));

            byte[] byteData = new byte[response.result_data_len];
            // NEEDS COPY? Or unsafe?
            Marshal.Copy(response.result_data, byteData, 0, (int) response.result_data_len);
            ProtobufInterop.flm_free_response(responsePtr);

            if (response.ffi_error)
            {
                var error = AGErrorProtobuf.Parser.ParseFrom(byteData);

                throw new AGOuterError(error);
            }

            return Configuration.Parser.ParseFrom(byteData);
        }

        internal static IntPtr InitFLM(Configuration configuration)
        {
            byte[] bytes = configuration.ToByteArray();

            IntPtr dataPointer = Marshal.AllocHGlobal(bytes.Length);

            try {
                Marshal.Copy(bytes, 0, dataPointer, bytes.Length);

                return ProtobufInterop.flm_init_protobuf(dataPointer, (ulong)bytes.Length);
            }
            finally
            {
                Marshal.FreeHGlobal(dataPointer);
            }
        }

        internal static byte[] CallRust(IntPtr FLMHandle, FFIMethod Method, byte[] Args)
        {
            IntPtr dataPointer = Marshal.AllocHGlobal(Args.Length);

            try
            {
                Marshal.Copy(Args, 0, dataPointer, Args.Length);

                IntPtr responsePtr = ProtobufInterop.flm_call_protobuf(FLMHandle, Method, dataPointer, (ulong)Args.Length);
                RustResponse response = (RustResponse)Marshal.PtrToStructure(responsePtr, typeof(RustResponse));
                
                byte[] byteData = new byte[response.result_data_len];
                Marshal.Copy(responsePtr, byteData, 0, (int) response.result_data_len);

                ProtobufInterop.flm_free_response(responsePtr);

                if (response.ffi_error)
                {
                    var error = AGErrorProtobuf.Parser.ParseFrom(byteData);

                    throw new AGOuterError(error);
                }

                return byteData;
            }
            finally
            {
                Marshal.FreeHGlobal(dataPointer);
            }
        }

        internal static void FreeFLMHandle(IntPtr handle)
        {
            ProtobufInterop.flm_free_handle(handle);
        }
    }
}
