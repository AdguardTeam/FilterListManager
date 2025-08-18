/// Creates a string from a literal or just an empty string
#[macro_export]
macro_rules! string {
    () => {
        String::new()
    };
    ($s:literal) => {
        String::from($s)
    };
    ($i:ident) => {
        String::from($i)
    };
}
