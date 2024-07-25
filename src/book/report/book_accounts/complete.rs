use crate::book::report::table::*;
use crate::book::*;

fn split_debit_credit<'l>(entries: &Vec<&'l BookEntry>) -> (Vec<&'l BookEntry>, Vec<&'l BookEntry>) {
    let mut debit = Vec::<&'l BookEntry>::new();
    let mut credit = Vec::<&'l BookEntry>::new();
    for entry in entries {
        match entry.amount {
            BookAmount::Debit(_) => debit.push(entry),
            BookAmount::Credit(_) => credit.push(entry)
        }
    }
    (debit, credit)
}

fn generate_account_desc(e: &BookEntry, accounts: &Book) -> String {
    format!("{} {}", e.account.0, accounts.naming.get(&e.account).map_or("", |v| v.as_str()))
}

fn generate_side(table: &mut Table, entry: Option<&&BookEntry>, accounts: &Book) {
    match entry {
        Some(e) => {
            table.insert(TableEntry::String(TableAlignment::Left, generate_account_desc(&e, accounts)));
            table.insert(TableEntry::String(TableAlignment::Right, format!("{:.2}", e.amount.plain_amount().0)));
        }
        None => {
            table.insert(TableEntry::Empty);
            table.insert(TableEntry::Empty);
        }
    }
}

pub fn generate_complete_accounts_table(filter: BookFilter, accounts: &Book) -> Table {
    let mut result : Table = Table::new();
    result.insert(TableEntry::String(TableAlignment::Left, "Date".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Verification Id".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Account".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Debit".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Account".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Credit".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Event".to_string()));
    result.insert(TableEntry::NewRow);
    for ((date, ledger_id), entry_list) in filter {
        result.insert(TableEntry::RowSeparator);
        result.insert(TableEntry::String(TableAlignment::Left, format!("{}", date)));
        result.insert(TableEntry::String(TableAlignment::Left, format!("{}", ledger_id)));
        let (debit_list, credit_list) = split_debit_credit(&entry_list);
        let mut debit_it = debit_list.iter();
        let mut credit_it = credit_list.iter();
        for row in 0..usize::MAX {
            let debit = debit_it.next();
            let credit = credit_it.next();
            if debit.is_none() && credit.is_none() {
                break;
            }
            if row!=0 {
                result.insert(TableEntry::Empty);
                result.insert(TableEntry::Empty);
            }
            generate_side(&mut result, debit, accounts);
            generate_side(&mut result, credit, accounts);
            if row==0 {
                result.insert(TableEntry::String(TableAlignment::Left, format!("{:?}", entry_list.get(0).unwrap().source_desc)));
            } else {
                result.insert(TableEntry::Empty);
            }
            result.insert(TableEntry::NewRow);
        }
    }
    result
}