namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterTypeActiveRulesInfo : FfiConverterRustBuffer<ActiveRulesInfo>
    {
        public static FfiConverterTypeActiveRulesInfo Instance = new FfiConverterTypeActiveRulesInfo();

        public override ActiveRulesInfo Read(BigEndianStream stream)
        {
            return new ActiveRulesInfo(
                filterId: FfiConverterInt32.Instance.Read(stream),
                groupId: FfiConverterInt32.Instance.Read(stream),
                isTrusted: FfiConverterBoolean.Instance.Read(stream),
                rules: FfiConverterSequenceString.Instance.Read(stream)
            );
        }

        public override int AllocationSize(ActiveRulesInfo value)
        {
            return
                FfiConverterInt32.Instance.AllocationSize(value.FilterId) +
                FfiConverterInt32.Instance.AllocationSize(value.GroupId) +
                FfiConverterBoolean.Instance.AllocationSize(value.IsTrusted) +
                FfiConverterSequenceString.Instance.AllocationSize(value.Rules);
        }

        public override void Write(ActiveRulesInfo value, BigEndianStream stream)
        {
            FfiConverterInt32.Instance.Write(value.FilterId, stream);
            FfiConverterInt32.Instance.Write(value.GroupId, stream);
            FfiConverterBoolean.Instance.Write(value.IsTrusted, stream);
            FfiConverterSequenceString.Instance.Write(value.Rules, stream);
        }
    }
}
