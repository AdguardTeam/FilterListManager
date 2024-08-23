namespace AdGuard.FilterListManager.MarshalLogic
{
    public class FilterTag
    {
        public FilterTag(int id, string keyword)
        {
            this.id = id;
            this.keyword = keyword;
        }

        public int id { get; set; }
        public string keyword { get; set; }

        public void Deconstruct(out int id, out string keyword)
        {
            id = this.id;
            keyword = this.keyword;
        }
    }
}