use super::types::*;

pub trait Converter {
    fn into_book(time::OffsetDateTime, Amount) -> BookAmount;
}
