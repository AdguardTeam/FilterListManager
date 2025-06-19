#[derive(Default)]
pub struct FiltersCompilationPolicy {
    /// List of constants for filters conditional compilation
    pub constants: Vec<String>,
}

impl FiltersCompilationPolicy {
    pub fn new(constants: Vec<String>) -> Self {
        Self { constants }
    }
}
