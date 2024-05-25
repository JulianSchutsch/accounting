use crate::book::event;

pub const foeretag:i32=1930;
pub const kundfordringar:i32=1510;
pub const foersaeljning_tjaenster:i32=3041;

pub fn foersaeljningskonto_from_category(category: event::income::Category) -> i32 {
    match category {
        event::income::Category::Services => foersaeljning_tjaenster,
    }
}
