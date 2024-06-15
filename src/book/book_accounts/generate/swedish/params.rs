use crate::book::*;

pub struct Params<'l1, 'l2, 'l2_1, 'l3, T>
where 'l1: 'l2_1,
      'l3: 'l2_1 {
    pub first: &'l1 phases::First,
    pub second: &'l2 mut phases::Second<'l2_1>,
    pub ledger_id: LedgerId,
    pub event: &'l3 T,
    pub event_ref: &'l3 Event,
}

pub struct IncompleteParams<'l1, 'l2, 'l2_1>
where 'l1: 'l2_1 {
    pub first: &'l1 phases::First,
    pub second: &'l2 mut phases::Second<'l2_1>,
    pub ledger_id: LedgerId,
}

impl<'l1, 'l2, 'l2_1> IncompleteParams<'l1, 'l2, 'l2_1> {
    pub fn complete_with<'t:'r, 'r, T>(self, event: &'t T, event_ref: &'t Event) -> Params<'l1, 'l2, 'l2_1, 'r, T>{
        Params{
            first: self.first,
            second: self.second,
            ledger_id: self.ledger_id,
            event,
            event_ref
        }
    }
}