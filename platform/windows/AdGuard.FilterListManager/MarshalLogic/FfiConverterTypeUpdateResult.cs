namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeUpdateResult : FfiConverterRustBuffer<UpdateResult>
    {
        public static FfiConverterTypeUpdateResult Instance = new FfiConverterTypeUpdateResult();

        public override UpdateResult Read(BigEndianStream stream)
        {
            return new UpdateResult(
                updatedList: FfiConverterSequenceTypeFullFilterList.Instance.Read(stream),
                remainingFiltersCount: FfiConverterInt32.Instance.Read(stream),
                filtersErrors: FfiConverterSequenceTypeUpdateFilterError.Instance.Read(stream)
            );
        }

        public override int AllocationSize(UpdateResult value)
        {
            return FfiConverterSequenceTypeFullFilterList.Instance.AllocationSize(value.updatedList)
                   + FfiConverterInt32.Instance.AllocationSize(value.remainingFiltersCount)
                   + FfiConverterSequenceTypeUpdateFilterError.Instance.AllocationSize(
                       value.filtersErrors
                   );
        }

        public override void Write(UpdateResult value, BigEndianStream stream)
        {
            FfiConverterSequenceTypeFullFilterList.Instance.Write(value.updatedList, stream);
            FfiConverterInt32.Instance.Write(value.remainingFiltersCount, stream);
            FfiConverterSequenceTypeUpdateFilterError.Instance.Write(value.filtersErrors, stream);
        }
    }
}