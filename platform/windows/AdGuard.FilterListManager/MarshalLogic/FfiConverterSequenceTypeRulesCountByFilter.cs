using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// List(RulesCountByFilter) to <see cref="RustBuffer"/> converter
    /// </summary>
    public class FfiConverterSequenceTypeRulesCountByFilter : FfiConverterRustBuffer<List<RulesCountByFilter>>
    {
        public static FfiConverterSequenceTypeRulesCountByFilter Instance = new FfiConverterSequenceTypeRulesCountByFilter();

        /// <inheritdoc/>
        public override List<RulesCountByFilter> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<RulesCountByFilter>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeRulesCountByFilter.Instance.Read(stream));
            }
            return result;
        }

        /// <inheritdoc/>
        public override int AllocationSize(List<RulesCountByFilter> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value.Select(item => FfiConverterTypeRulesCountByFilter.Instance.AllocationSize(item)).Sum();
            return sizeForLength + sizeForItems;
        }

        /// <inheritdoc/>
        public override void Write(List<RulesCountByFilter> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeRulesCountByFilter.Instance.Write(item, stream));
        }
    }
}