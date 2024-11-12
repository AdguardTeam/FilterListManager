using System.Collections.Generic;
using System.Linq;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// List(DisabledRulesRaw) to <see cref="RustBuffer"/> converter
    /// </summary>
    public class FfiConverterSequenceTypeDisabledRulesRaw : FfiConverterRustBuffer<List<DisabledRulesRaw>>
    {
        public static FfiConverterSequenceTypeDisabledRulesRaw Instance = new FfiConverterSequenceTypeDisabledRulesRaw();

        /// <inheritdoc/>
        public override List<DisabledRulesRaw> Read(BigEndianStream stream)
        {
            var length = stream.ReadInt();
            var result = new List<DisabledRulesRaw>(length);
            for (int i = 0; i < length; i++)
            {
                result.Add(FfiConverterTypeDisabledRulesRaw.Instance.Read(stream));
            }
            return result;
        }

        /// <inheritdoc/>
        public override int AllocationSize(List<DisabledRulesRaw> value)
        {
            var sizeForLength = 4;

            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                return sizeForLength;
            }

            var sizeForItems = value.Select(item => FfiConverterTypeDisabledRulesRaw.Instance.AllocationSize(item)).Sum();
            return sizeForLength + sizeForItems;
        }

        /// <inheritdoc/>
        public override void Write(List<DisabledRulesRaw> value, BigEndianStream stream)
        {
            // details/1-empty-list-as-default-method-parameter.md
            if (value == null)
            {
                stream.WriteInt(0);
                return;
            }

            stream.WriteInt(value.Count);
            value.ForEach(item => FfiConverterTypeDisabledRulesRaw.Instance.Write(item, stream));
        }
    }
}