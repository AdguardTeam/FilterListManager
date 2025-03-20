using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    internal class FfiConverterSequenceTypeActiveRulesInfo : FfiConverterRustBuffer<List<ActiveRulesInfo>>
    {
        public static FfiConverterSequenceTypeActiveRulesInfo Instance = new FfiConverterSequenceTypeActiveRulesInfo();

        public override List<ActiveRulesInfo> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<ActiveRulesInfo>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeActiveRulesInfo.Instance.Read(stream));
            }
            return result;
        }

        public override int AllocationSize(List<ActiveRulesInfo> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value.Select(item => FfiConverterTypeActiveRulesInfo.Instance.AllocationSize(item)).Sum();
            return sizeForLength + sizeForItems;
        }

        public override void Write(List<ActiveRulesInfo> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeActiveRulesInfo.Instance.Write(item, stream));
        }
    }
}
