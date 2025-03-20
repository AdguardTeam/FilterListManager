using System;
using System.Runtime.InteropServices;
using AdGuard.FilterListManager.Api;
using AdGuard.FilterListManager.Api.Exceptions;
using AdGuard.Utils.Base.Interop;
using AGErrorProtobuf = FilterListManager.AGOuterError;

namespace AdGuard.FilterListManager.RustInterface
{
    static class ProtobufBridge
    {
        internal static RustResponseResult CallRust(
            IntPtr flmHandle, 
            FfiMethod ffiMethod, 
            byte[] args, 
            Func<IntPtr, FfiMethod, IntPtr, ulong, IntPtr> flmInteropFunc)
        {
            IntPtr pArgs = IntPtr.Zero;
            try
            {
                pArgs = Marshal.AllocHGlobal(args.Length);
                Marshal.Copy(args, 0, pArgs, args.Length);
                RustResponseResult rustResponseResult = GetRustResponseResult(
                    () => 
                        flmInteropFunc(
                            flmHandle, 
                            ffiMethod, 
                            pArgs, 
                            (ulong)args.Length));
                return rustResponseResult;
            }
            finally
            {
                MarshalUtils.SafeFreeHGlobal(pArgs);
            }
        }
        
        private static RustResponseResult GetRustResponseResult(Func<IntPtr> nativeRustFunc)
        {
            IntPtr pRustResponse = IntPtr.Zero;
            try
            {
                pRustResponse = nativeRustFunc();
                RustResponse rustResponse = MarshalUtils.PtrToStructure<RustResponse>(pRustResponse);
                uint resultDataLen = rustResponse.ResultDataLen.ToUInt32();
                byte[] resultDataBuffer = new byte[resultDataLen];
                Marshal.Copy(rustResponse.PResultData, resultDataBuffer, 0, (int)resultDataLen);
                if (rustResponse.FfiError)
                {
                    // fi_error is set to true if an error occurs directly in the interface between languages.
                    // In this case, we are just throwing a business logic error.
                    // In other words, the RUST - constructor throws an exception.
                    // see more https://bit.int.agrd.dev/projects/ADGUARD-CORE-LIBS/repos/filter-list-manager/pull-requests/156/overview?commentId=344275
                    AGErrorProtobuf error = AGErrorProtobuf.Parser.ParseFrom(resultDataBuffer);
                    throw new AgOuterException(error);
                }
                
                RustResponseResult rustResponseResult = new RustResponseResult
                {
                    HandlePointer = rustResponse.PResultData,
                    Discriminant = rustResponse.Discriminant,
                    Buffer = resultDataBuffer
                };
                    
                return rustResponseResult;
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
