using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// List(FilterListRulesRaw) to <see cref="RustBuffer"/> converter
    /// </summary>
    public class FfiConverterSequenceTypeFilterListRulesRaw : FfiConverterRustBuffer<List<FilterListRulesRaw>>
    {
        public static FfiConverterSequenceTypeFilterListRulesRaw Instance = new FfiConverterSequenceTypeFilterListRulesRaw();

        /// <inheritdoc/>
        public override List<FilterListRulesRaw> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<FilterListRulesRaw>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeFilterListRulesRaw.Instance.Read(stream));
            }
            return result;
        }

        /// <inheritdoc/>
        public override int AllocationSize(List<FilterListRulesRaw> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value.Select(item => FfiConverterTypeFilterListRulesRaw.Instance.AllocationSize(item)).Sum();
            return sizeForLength + sizeForItems;
        }

        /// <inheritdoc/>
        public override void Write(List<FilterListRulesRaw> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeFilterListRulesRaw.Instance.Write(item, stream));
        }
    }
}
