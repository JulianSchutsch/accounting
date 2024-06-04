use crate::book::bookaccounts::Accounts;
use crate::book::converter::Converter;
use crate::book::ledger::{Event, LedgerId};

pub struct Params<'l1, 'l1_1, 'l2, 'l3, T>
where
    'l3:'l1_1
{
    pub accounts: &'l1 mut Accounts<'l1_1>,
    pub converter: &'l2 dyn Converter,
    pub ledger_id: LedgerId,
    pub event: &'l3 T,
    pub event_ref: &'l3 Event,
}

pub struct IncompleteParams<'l1, 'l1_1, 'l2> {
    pub ledger_id: LedgerId,
    pub accounts: &'l1 mut Accounts<'l1_1>,
    pub converter: &'l2 dyn Converter,
}

impl<'l1, 'l1_1, 'l2> IncompleteParams<'l1, 'l1_1, 'l2> {
    pub fn complete_with<'t:'r, 'r, T>(self, event: &'t T, event_ref: &'t Event) -> Params<'l1, 'l1_1, 'l2, 'r, T>{
        Params{
            accounts: self.accounts,
            converter: self.converter,
            ledger_id: self.ledger_id,
            event: event,
            event_ref: event_ref
        }
    }
}