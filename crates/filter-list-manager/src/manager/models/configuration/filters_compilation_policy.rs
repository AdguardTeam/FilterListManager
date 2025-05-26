#[derive(Default)]
pub struct FiltersCompilationPolicy {
    pub constants: Vec<String>,
}

impl FiltersCompilationPolicy {
    pub fn new(constants: Vec<String>) -> Self {
        Self { constants }
    }
}
