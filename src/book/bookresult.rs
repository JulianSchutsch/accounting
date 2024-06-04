use std::collections::LinkedList;
use std::fmt;

#[derive(Debug)]
pub struct BookError {
    errors: LinkedList<String>,
}

impl BookError {
    pub fn new<T: ToString>(s:T) -> BookError {
        let mut list = LinkedList::<String>::new();
        list.push_front(s.to_string());
        BookError{
            errors: list
        }
    }

    pub fn new_from<E: ToString, T: ToString>(e: E, s:T) -> BookError {
        let mut list = LinkedList::<String>::new();
        list.push_front(e.to_string());
        list.push_front(s.to_string());
        BookError{
            errors: list
        }
    }

    pub fn extend<T: ToString>(self, s:T) -> Self {
        let mut result = self;
        result.errors.push_front(s.to_string());
        result
    }
}

impl<T> From<T> for BookError where T: std::error::Error {
    fn from(value: T) -> Self {
        BookError::new(value)
    }
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ident : usize = 0;
        for error in self.errors.iter() {
            writeln!(f, "{: <ident$}{}", "", error, ident=ident)?;
            ident += 2;
        }
        Ok(())
    }
}

pub type BookResult<T=()> = Result<T, BookError>;

