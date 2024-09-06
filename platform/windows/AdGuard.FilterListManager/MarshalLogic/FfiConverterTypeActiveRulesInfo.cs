namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeActiveRulesInfo : FfiConverterRustBuffer<ActiveRulesInfo>
    {
        public static FfiConverterTypeActiveRulesInfo INSTANCE = new FfiConverterTypeActiveRulesInfo();

        public override ActiveRulesInfo Read(BigEndianStream stream)
        {
            return new ActiveRulesInfo(
                filterId: FfiConverterInt64.INSTANCE.Read(stream),
                groupId: FfiConverterInt32.INSTANCE.Read(stream),
                isTrusted: FfiConverterBoolean.INSTANCE.Read(stream),
                rules: FfiConverterSequenceString.INSTANCE.Read(stream)
            );
        }

        public override int AllocationSize(ActiveRulesInfo value)
        {
            return
                FfiConverterInt64.INSTANCE.AllocationSize(value.FilterId) +
                FfiConverterInt32.INSTANCE.AllocationSize(value.GroupId) +
                FfiConverterBoolean.INSTANCE.AllocationSize(value.IsTrusted) +
                FfiConverterSequenceString.INSTANCE.AllocationSize(value.Rules);
        }

        public override void Write(ActiveRulesInfo value, BigEndianStream stream)
        {
            FfiConverterInt64.INSTANCE.Write(value.FilterId, stream);
            FfiConverterInt32.INSTANCE.Write(value.GroupId, stream);
            FfiConverterBoolean.INSTANCE.Write(value.IsTrusted, stream);
            FfiConverterSequenceString.INSTANCE.Write(value.Rules, stream);
        }
    }
}
