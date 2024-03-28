#[derive(Debug, Clone, PartialEq,Eq)]
pub struct Question(String);
impl Question {
    pub fn new(text: String) -> Self {
        Question(text)
    }
    pub fn text(&self) -> &str {
        &self.0
    }
}