use crate::book::event;

use crate::book::accounts::AccountId as Id;

pub const foeretag:Id = Id(1930);
pub const kundfordringar:Id = Id(1510);
pub const foersaeljning_tjaenster:Id = Id(3041);

pub fn foersaeljningskonto_from_category(category: event::income::Category) -> Id {
    match category {
        event::income::Category::Services => foersaeljning_tjaenster,
    }
}
