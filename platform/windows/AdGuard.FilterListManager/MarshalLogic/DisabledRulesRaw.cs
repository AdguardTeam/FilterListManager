namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Raw version of disabled rule entity
    /// </summary>
    public class DisabledRulesRaw
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="DisabledRulesRaw"/> class.
        /// </summary>
        /// <param name="filterId">The filter identifier.</param>
        /// <param name="text">The rule text.</param>
        public DisabledRulesRaw(long filterId, string text)
        {
            FilterId = filterId;
            Text = text;
        }

        /// <summary>
        /// Gets the filter identifier.
        /// </summary>
        public long FilterId { get; }

        /// <summary>
        /// Gets the rule text.
        /// </summary>
        public string Text { get; }
    }
}