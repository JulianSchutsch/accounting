use super::date::Date;

#[derive(Clone, Copy)]
pub struct Period {
    pub begin : Date,
    pub end: Date
}

impl Period {
    pub fn new() -> Self {
        Self{begin: Date::MIN, end: Date::MIN}
    }
}