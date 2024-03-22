use uuid::Uuid;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum UserIdError {
    #[error("{0} is not a valid uuid.")]
    InvailedValue(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserId{
    value: Uuid,
}
impl UserId{
    pub fn new() -> Self{
        Self{
            value: Uuid::new_v4()
        }
    }
}


impl ToString for UserId{
    fn to_string(&self) -> String{
        self.value.to_string()
    }
}
impl TryFrom<&str> for UserId {
    type Error = UserIdError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = Uuid::parse_str(value).map_err(|_| UserIdError::InvailedValue(value.to_string()))?;
        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let id = UserId::new();
        assert_eq!(id.to_string().len(), 36);
    }
    #[test]
    fn test_from_string() {
        let id = UserId::try_from("00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(id.to_string(), "00000000-0000-0000-0000-000000000000");
    }
    #[test]
    fn test_from_string_error() {
        let id = UserId::try_from("00000000-0000-0000-0000-00000000000");
        assert!(id.is_err());
    }
}