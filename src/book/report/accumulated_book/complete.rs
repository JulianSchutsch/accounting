use crate::book::report::table::*;
use crate::book::*;

pub fn generate_complete_accounts_table(accumulated_book: &AccumulatedBook, accounts: &Book) -> Table {
    let mut result : Table = Table::new();
    result.insert(TableEntry::String(TableAlignment::Left, "Account".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Accumulated".to_string()));
    result.insert(TableEntry::NewRow);
    result.insert(TableEntry::RowSeparator);
    for (key, value) in accumulated_book.values.iter() {
        result.insert(TableEntry::String(TableAlignment::Left, format!("{} {}", key, accounts.naming.get(key).map_or("", |v| v.as_str()))));
        result.insert(TableEntry::String(TableAlignment::Left, format!("{}", value)));
        result.insert(TableEntry::NewRow);
    }
    result
}