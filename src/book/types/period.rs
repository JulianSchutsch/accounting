use super::date::Date;

#[derive(Clone, Copy)]
pub struct Period {
    pub begin : Date,
    pub end: Date
}