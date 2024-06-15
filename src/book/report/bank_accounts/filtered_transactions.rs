use crate::book::report::table::*;
use crate::book::*;

pub fn generate(banks: &BankAccounts, filter: BankTransactionsFilterBuilder, consumed: Option<&BankTransactionConsumer>) -> Table {
    let mut table = Table::new();
    for (_, account) in banks.iter() {
        table.insert(TableEntry::TitleRow(account.description()));
        for entry in filter.iter(account, consumed) {
            table.insert(TableEntry::String(TableAlignment::Right, format!("{}", entry.amount)));
            table.insert(TableEntry::String(TableAlignment::Left, entry.description()));
            table.insert(TableEntry::NewRow);
        }
    }
    table
}