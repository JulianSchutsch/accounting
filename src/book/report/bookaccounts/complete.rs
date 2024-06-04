use crate::book::report::table::*;
use crate::book::bookaccounts::*;

fn split_debit_credit<'l>(entries: &Vec<AccountEntry<'l>>) -> (Vec<AccountEntry<'l>>, Vec<AccountEntry<'l>>) {
    let mut debit = Vec::<AccountEntry>::new();
    let mut credit = Vec::<AccountEntry>::new();
    for entry in entries {
        match entry.amount {
            AccountAmount::Debit(_) => debit.push(entry.clone()),
            AccountAmount::Credit(_) => credit.push(entry.clone())
        }
    }
    (debit, credit)
}

pub fn generate_account_desc(e: &AccountEntry, accounts: &BookAccounts) -> String {
    format!("{} {}", e.account.0, accounts.naming.get(&e.account).map_or("", |v| v.as_str()))
}

pub fn generate_side(table: &mut Table, entry: Option<&AccountEntry>, accounts: &BookAccounts) {
    match entry {
        Some(e) => {
            table.insert(TableEntry::String(TableAlignment::Left, generate_account_desc(&e, accounts)));
            table.insert(TableEntry::String(TableAlignment::Right, format!("{}", e.amount.plain_amount().0)));
        }
        None => {
            table.insert(TableEntry::Empty);
            table.insert(TableEntry::Empty);
        }
    }
}

pub fn generate_complete_accounts_table(accounts: &BookAccounts) -> Table {
    let mut result : Table = Table::new();
    result.insert(TableEntry::String(TableAlignment::Left, "Date".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Verification Id".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Account".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Debit".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Account".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Credit".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Event".to_string()));
    result.insert(TableEntry::NewRow);
    for ((date, ledger_id), entry_list) in accounts.iter() {
        result.insert(TableEntry::RowSeparator);
        result.insert(TableEntry::String(TableAlignment::Left, format!("{}", date)));
        result.insert(TableEntry::String(TableAlignment::Left, format!("{}", ledger_id.0)));
        let (debit_list, credit_list) = split_debit_credit(entry_list);
        let mut debit_it = debit_list.iter();
        let mut credit_it = credit_list.iter();
        for row in 0..9999999 {
            let debit = debit_it.next();
            let credit = credit_it.next();
            if(debit.is_none() && credit.is_none()) {
                break;
            }
            if(row!=0) {
                result.insert(TableEntry::Empty);
                result.insert(TableEntry::Empty);
            }
            generate_side(&mut result, debit, accounts);
            generate_side(&mut result, credit, accounts);
            if(row==0) {
                result.insert(TableEntry::String(TableAlignment::Left, format!("{:?}", entry_list.get(0).unwrap().source.id())));
            } else {
                result.insert(TableEntry::Empty);
            }
            result.insert(TableEntry::NewRow);
        }
    }
    result
}