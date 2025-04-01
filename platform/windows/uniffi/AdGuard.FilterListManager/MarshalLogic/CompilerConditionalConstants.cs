using System.Collections.Generic;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// CompilerConditionalConstants class for Configuration <see cref="Configuration"/>
    /// </summary>
    public class CompilerConditionalConstants
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="CompilerConditionalConstants"/> class.
        /// </summary>
        /// <param name="compilerConditionalConstants">The compiler conditional constants.</param>
        public CompilerConditionalConstants(List<string> compilerConditionalConstants)
        {
            CompilerConditionalConstants = compilerConditionalConstants;
        }

        /// <summary>
        /// Gets or sets the filter compiler conditional constants.
        /// </summary>
        public List<string> CompilerConditionalConstants { get; set; }
    }

}
