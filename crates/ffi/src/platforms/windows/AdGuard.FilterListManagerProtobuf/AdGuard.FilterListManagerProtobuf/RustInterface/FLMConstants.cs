using System.Runtime.InteropServices;

namespace AdGuard.FilterListManagerProtobuf.RustInterface
{
    /// <summary>
    /// Filter List Manager public constants
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct FLMConstants
    {
        /// <summary>
        /// Filter ID for *User rules* filter
        /// </summary>
        public int UserRulesId;

        /// <summary>
        /// Group ID for special *custom filters group*
        /// </summary>
        public int CustomGroupId;

        /// <summary>
        /// Group ID for *special service filters*
        /// </summary>
        public int SpecialGroupId;

        /// <summary>
        /// Smallest possible filter_id. You can safely occupy any filter with an id lower than this number.
        /// The library is guaranteed to never create a filter with this id
        /// </summary>
        public int SmallestFilterId;
    }
}
