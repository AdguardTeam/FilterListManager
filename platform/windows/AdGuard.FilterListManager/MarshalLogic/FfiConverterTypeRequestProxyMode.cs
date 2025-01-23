namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeRequestProxyMode : FfiConverterRustBuffer<RequestProxyMode>
    {
        public static FfiConverterRustBuffer<RequestProxyMode> Instance = new FfiConverterTypeRequestProxyMode();

        public override RequestProxyMode Read(BigEndianStream stream)
        {
            var value = stream.ReadInt();
            switch (value)
            {
                case 1:
                    return new RequestProxyMode.UseSystemProxy(
                    );
                case 2:
                    return new RequestProxyMode.NoProxy(
                    );
                case 3:
                    return new RequestProxyMode.UseCustomProxy(
                        FfiConverterString.Instance.Read(stream)
                    );
                default:
                    throw new InternalException(
                        $"invalid enum value '{value}' in FfiConverterTypeRequestProxyMode.Read()");
            }
        }

        public override int AllocationSize(RequestProxyMode value)
        {
            switch (value)
            {
                case RequestProxyMode.UseSystemProxy variantValue:
                    return 4;
                case RequestProxyMode.NoProxy variantValue:
                    return 4;
                case RequestProxyMode.UseCustomProxy variantValue:
                    return 4
                        + FfiConverterString.Instance.AllocationSize(variantValue.Address);
                default:
                    throw new InternalException(
                        $"invalid enum value '{value}' in FfiConverterTypeRequestProxyMode.AllocationSize()");
            }
        }

        public override void Write(RequestProxyMode value, BigEndianStream stream)
        {
            switch (value)
            {
                case RequestProxyMode.UseSystemProxy variantValue:
                    stream.WriteInt(1);
                    break;
                case RequestProxyMode.NoProxy variantValue:
                    stream.WriteInt(2);
                    break;
                case RequestProxyMode.UseCustomProxy variantValue:
                    stream.WriteInt(3);
                    FfiConverterString.Instance.Write(variantValue.Address, stream);
                    break;
                default:
                    throw new InternalException(
                        $"invalid enum value '{value}' in FfiConverterTypeRequestProxyMode.Write()");
            }
        }
    }
}
