use crate::book::*;

pub struct Params<'l1, 'l1_1, 'l2, 'l3, 'l3_1, 'l4, 'l5, T>
where
    'l5:'l1_1,
    'l2:'l3_1
{
    pub accounts: &'l1 mut BookAccounts<'l1_1>,
    pub bank_accounts: &'l2 BankAccounts,
    pub bank_transaction_consumer: &'l3 mut BankTransactionConsumer<'l3_1>,
    pub import: &'l4 Import,
    pub ledger_id: LedgerId,
    pub event: &'l5 T,
    pub event_ref: &'l5 Event,
}

pub struct IncompleteParams<'l1, 'l1_1, 'l2, 'l3, 'l3_1, 'l4>
where 'l2:'l3_1 {
    pub ledger_id: LedgerId,
    pub accounts: &'l1 mut BookAccounts<'l1_1>,
    pub bank_accounts: &'l2 BankAccounts,
    pub bank_transaction_consumer: &'l3 mut BankTransactionConsumer<'l3_1>,
    pub import: &'l4 Import,
}

impl<'l1, 'l1_1, 'l2, 'l3, 'l3_1, 'l4> IncompleteParams<'l1, 'l1_1, 'l2, 'l3, 'l3_1, 'l4> {
    pub fn complete_with<'t:'r, 'r, T>(self, event: &'t T, event_ref: &'t Event) -> Params<'l1, 'l1_1, 'l2, 'l3, 'l3_1, 'l4, 'r, T>{
        Params{
            accounts: self.accounts,
            bank_accounts: self.bank_accounts,
            bank_transaction_consumer: self.bank_transaction_consumer,
            import: self.import,
            ledger_id: self.ledger_id,
            event,
            event_ref
        }
    }
}