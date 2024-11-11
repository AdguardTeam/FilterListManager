namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeAgOuterException
        : FfiConverterRustBuffer<AgOuterException>,
            ICallStatusErrorHandler<AgOuterException>
    {
        public static FfiConverterTypeAgOuterException Instance =
            new FfiConverterTypeAgOuterException();

        public override AgOuterException Read(BigEndianStream stream)
        {
            var value = stream.ReadInt();
            switch (value)
            {
                case 1:
                    return new AgOuterException.CannotOpenDatabaseException(
                        FfiConverterString.Instance.Read(stream)
                    );
                case 2:
                    return new AgOuterException.NotADatabaseException(FfiConverterString.Instance.Read(stream));
                case 3:
                    return new AgOuterException.DiskFullException(FfiConverterString.Instance.Read(stream));
                case 4:
                    return new AgOuterException.DatabaseBusy(FfiConverterString.Instance.Read(stream));
                case 5:
                    return new AgOuterException.EntityNotFoundException(
                        FfiConverterString.Instance.Read(stream)
                    );
                case 6:
                    return new AgOuterException.PathNotFoundException(FfiConverterString.Instance.Read(stream));
                case 7:
                    return new AgOuterException.PathHasDeniedPermissionException(
                        FfiConverterString.Instance.Read(stream)
                    );
                case 8:
                    return new AgOuterException.PathAlreadyExistsException(
                        FfiConverterString.Instance.Read(stream)
                    );
                case 9:
                    return new AgOuterException.TimedOutException(FfiConverterString.Instance.Read(stream));
                case 10:
                    return new AgOuterException.HttpClientNetworkException(
                        FfiConverterString.Instance.Read(stream)
                    );
                case 11:
                    return new AgOuterException.HttpClientBodyRecoveryFailedException(
                        FfiConverterString.Instance.Read(stream)
                    );
                case 12: return new AgOuterException.HttpStrict200Response(FfiConverterString.Instance.Read(stream));
                case 13:
                    return new AgOuterException.FilterContentIsLikelyNotAFilter(
                        FfiConverterString.Instance.Read(stream));
                case 14:
                    return new AgOuterException.FilterParserException(
                        FfiConverterString.Instance.Read(stream)
                    );
                case 15:
                    return new AgOuterException.FieldIsEmptyException(FfiConverterString.Instance.Read(stream));
                case 16:
                    return new AgOuterException.MutexException(FfiConverterString.Instance.Read(stream));
                case 17:
                    return new AgOuterException.OtherException(FfiConverterString.Instance.Read(stream));
                default:
                    throw new InternalException(
                        $"invalid error value '{value}' in FfiConverterTypeAgOuterException.Read()"
                    );
            }
        }

        public override int AllocationSize(AgOuterException value)
        {
            return 4 + FfiConverterString.Instance.AllocationSize(value.Message);
        }

        public override void Write(AgOuterException value, BigEndianStream stream)
        {
            if (value is AgOuterException.CannotOpenDatabaseException)
                stream.WriteInt(1);
            else if (value is AgOuterException.NotADatabaseException)
                stream.WriteInt(2);
            else if (value is AgOuterException.DiskFullException)
                stream.WriteInt(3);
            else if (value is AgOuterException.DatabaseBusy)
                stream.WriteInt(4);
            else if (value is AgOuterException.EntityNotFoundException)
                stream.WriteInt(5);
            else if (value is AgOuterException.PathNotFoundException)
                stream.WriteInt(6);
            else if (value is AgOuterException.PathHasDeniedPermissionException)
                stream.WriteInt(7);
            else if (value is AgOuterException.PathAlreadyExistsException)
                stream.WriteInt(8);
            else if (value is AgOuterException.TimedOutException)
                stream.WriteInt(9);
            else if (value is AgOuterException.HttpClientNetworkException)
                stream.WriteInt(10);
            else if (value is AgOuterException.HttpClientBodyRecoveryFailedException)
                stream.WriteInt(11);
            else if (value is AgOuterException.HttpStrict200Response)
                stream.WriteInt(12);
            else if (value is AgOuterException.FilterContentIsLikelyNotAFilter)
                stream.WriteInt(13);
            else if (value is AgOuterException.FilterParserException)
                stream.WriteInt(14);
            else if (value is AgOuterException.FieldIsEmptyException)
                stream.WriteInt(15);
            else if (value is AgOuterException.MutexException)
                stream.WriteInt(16);
            else if (value is AgOuterException.OtherException)
                stream.WriteInt(17);
            else
                throw new InternalException(
                    $"invalid error value '{value}' in FfiConverterTypeAgOuterException.Write()"
                );
        }
    }
}