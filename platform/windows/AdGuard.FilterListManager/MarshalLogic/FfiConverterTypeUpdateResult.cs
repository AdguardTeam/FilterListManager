namespace AdGuard.FilterListManager.MarshalLogic
{
    class FfiConverterTypeUpdateResult : FfiConverterRustBuffer<UpdateResult>
    {
        public static FfiConverterTypeUpdateResult INSTANCE = new FfiConverterTypeUpdateResult();

        public override UpdateResult Read(BigEndianStream stream)
        {
            return new UpdateResult(
                updatedList: FfiConverterSequenceTypeFullFilterList.INSTANCE.Read(stream),
                remainingFiltersCount: FfiConverterInt32.INSTANCE.Read(stream),
                filtersErrors: FfiConverterSequenceTypeUpdateFilterError.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(UpdateResult value)
        {
            return FfiConverterSequenceTypeFullFilterList.INSTANCE.AllocationSize(value.updatedList)
                   + FfiConverterInt32.INSTANCE.AllocationSize(value.remainingFiltersCount)
                   + FfiConverterSequenceTypeUpdateFilterError.INSTANCE.AllocationSize(
                       value.filtersErrors
                   );
        }

        public override void Write(UpdateResult value, BigEndianStream stream)
        {
            FfiConverterSequenceTypeFullFilterList.INSTANCE.Write(value.updatedList, stream);
            FfiConverterInt32.INSTANCE.Write(value.remainingFiltersCount, stream);
            FfiConverterSequenceTypeUpdateFilterError.INSTANCE.Write(value.filtersErrors, stream);
        }
    }
}