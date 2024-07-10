use crate::book::*;

#[derive(Debug)]
pub enum Event {
    Income(Income),
    Invoice(Invoice),
    Salary(Salary)
}

impl Event {
    pub fn date(&self) -> Date {
        match self {
            Event::Income(e) => e.date,
            Event::Invoice(e) => e.date,
            Event::Salary(e) => e.date,
        }
    }

    pub fn id(&self) -> &String {
        match self {
            Event::Income(e) => &e.id,
            Event::Invoice(e) => &e.id,
            Event::Salary(e) => &e.id,
        }
    }
}
