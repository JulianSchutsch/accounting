use crate::book::*;

pub struct Params<'l1, 'l1_1, 'l2, 'l3, T>
where
    'l3:'l1_1
{
    pub accounts: &'l1 mut BookAccounts<'l1_1>,
    pub import: &'l2 Import,
    pub ledger_id: LedgerId,
    pub event: &'l3 T,
    pub event_ref: &'l3 Event,
}

pub struct IncompleteParams<'l1, 'l1_1, 'l2> {
    pub ledger_id: LedgerId,
    pub accounts: &'l1 mut BookAccounts<'l1_1>,
    pub import: &'l2 Import,
}

impl<'l1, 'l1_1, 'l2> IncompleteParams<'l1, 'l1_1, 'l2> {
    pub fn complete_with<'t:'r, 'r, T>(self, event: &'t T, event_ref: &'t Event) -> Params<'l1, 'l1_1, 'l2, 'r, T>{
        Params{
            accounts: self.accounts,
            import: self.import,
            ledger_id: self.ledger_id,
            event,
            event_ref
        }
    }
}