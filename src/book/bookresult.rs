#[derive(Debug)]
pub struct BookError {
    value: String,
}

impl BookError {
    pub fn new_from_str(s:&str) -> BookError {
        BookError{
            value: String::from(s)
        }
    }
    pub fn extend_by_str(&self, s:& str) -> BookError {
        BookError{
            value: format!("{} \n {}", self.value.as_str(), s)
        }
    }
}

impl From<&str> for BookError {
    fn from(value: & str) -> Self {
        return BookError::new_from_str(value);
    }
}

impl From<serde_yaml::Error> for BookError {
    fn from(value: serde_yaml::Error) -> Self {
        BookError{ value: value.to_string() }
    }
}

pub type BookResult<T=()> = Result<T, BookError>;