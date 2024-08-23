namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FilterGroup
    {
        public FilterGroup(int id, string name, int displayNumber)
        {
            this.id = id;
            this.name = name;
            this.displayNumber = displayNumber;
        }

        public int id { get; set; }
        public string name { get; set; }
        public int displayNumber { get; set; }

        public void Deconstruct(out int id, out string name, out int displayNumber)
        {
            id = this.id;
            name = this.name;
            displayNumber = this.displayNumber;
        }
    }
}