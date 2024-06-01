use std::collections::HashMap;
use crate::book::report::table::*;
use crate::book::bookaccounts::*;
use crate::book::types::*;

fn generate_complete_accounts_table(accounts: &Accounts, naming: &dyn AccountNaming) {
    let mut result : Table = Table::new();
    result.insert(TableEntry::String(TableAlignment::Left, "Complete".to_string()));
    let mut rows_per_date = HashMap::<Date, usize>::new();
    let previous_date = Date::default();
    for (date, entry_list) in accounts.iter() {

    }
}