#[derive(Debug,Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl Id {
    pub fn new() -> Id {
        Id("".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}