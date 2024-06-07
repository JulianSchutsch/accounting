use crate::book::*;

pub struct Params<'l1, 'l1_1, 'l2, 'l3, 'l4, T>
where
    'l4:'l1_1
{
    pub accounts: &'l1 mut BookAccounts<'l1_1>,
    pub bank_accounts: &'l2 BankAccounts,
    pub import: &'l3 Import,
    pub ledger_id: LedgerId,
    pub event: &'l4 T,
    pub event_ref: &'l4 Event,
}

pub struct IncompleteParams<'l1, 'l1_1, 'l2, 'l3> {
    pub ledger_id: LedgerId,
    pub accounts: &'l1 mut BookAccounts<'l1_1>,
    pub bank_accounts: &'l2 BankAccounts,
    pub import: &'l3 Import,
}

impl<'l1, 'l1_1, 'l2, 'l3> IncompleteParams<'l1, 'l1_1, 'l2, 'l3> {
    pub fn complete_with<'t:'r, 'r, T>(self, event: &'t T, event_ref: &'t Event) -> Params<'l1, 'l1_1, 'l2, 'l3, 'r, T>{
        Params{
            accounts: self.accounts,
            bank_accounts: self.bank_accounts,
            import: self.import,
            ledger_id: self.ledger_id,
            event,
            event_ref
        }
    }
}