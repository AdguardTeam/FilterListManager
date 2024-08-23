namespace AdGuard.FilterListManager.MarshalLogic
{
    public class UpdateFilterError
    {
        public UpdateFilterError(long filterId, string message)
        {
            this.filterId = filterId;
            this.message = message;
        }

        public long filterId { get; set; }
        public string message { get; set; }

        public void Deconstruct(out long filterId, out string message)
        {
            filterId = this.filterId;
            message = this.message;
        }
    }
}