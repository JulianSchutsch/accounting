use crate::book::*;
use crate::book::book_accounts::generate::swedish::active_associables::ActiveAssociables;

pub struct Params<'l1, 'l2, 'l2_1, 'l3, 'l4, T>
where 'l1: 'l2_1,
      'l3: 'l2_1 {
    pub first: &'l1 phases::First,
    pub second: &'l2 mut phases::Second<'l2_1>,
    pub ledger_id: LedgerId,
    pub event: &'l3 T,
    pub event_ref: &'l3 Event,
    pub associables: &'l4 mut ActiveAssociables
}

pub struct IncompleteParams<'l1, 'l2, 'l2_1, 'l3>
where 'l1: 'l2_1 {
    pub first: &'l1 phases::First,
    pub second: &'l2 mut phases::Second<'l2_1>,
    pub ledger_id: LedgerId,
    pub associables: &'l3 mut ActiveAssociables
}

impl<'l1, 'l2, 'l2_1, 'l3> IncompleteParams<'l1, 'l2, 'l2_1, 'l3> {
    pub fn complete_with<'t:'r, 'r, T>(self, event: &'t T, event_ref: &'t Event) -> Params<'l1, 'l2, 'l2_1, 'r, 'l3, T>{
        Params{
            first: self.first,
            second: self.second,
            ledger_id: self.ledger_id,
            event,
            event_ref,
            associables: self.associables
        }
    }
}