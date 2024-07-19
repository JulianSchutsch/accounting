use crate::book::report::table::*;
use crate::book::*;

pub fn generate(banks: &BankAccounts, filter: BankTransactionsFilterBuilder) -> Table {
    let mut table = Table::new();
    for (_, account) in banks.iter() {
        table.insert(TableEntry::TitleRow(account.description()));
        for entry in filter.iter(account) {
            table.insert(TableEntry::String(TableAlignment::Right, format!("{}", entry.amount)));
            table.insert(TableEntry::String(TableAlignment::Left, entry.description()));
            table.insert(TableEntry::NewRow);
        }
    }
    table
}