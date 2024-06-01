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

pub fn generate_account_desc(e: &AccountEntry, naming: &dyn AccountNaming) -> String {
    format!("{} {}", e.account.0, naming.name(e.account).unwrap_or_else(|| "".to_string()))
}

pub fn generate_side(table: &mut Table, entry: Option<&AccountEntry>, naming: &dyn AccountNaming) {
    match entry {
        Some(e) => {
            table.insert(TableEntry::String(TableAlignment::Left, generate_account_desc(&e, naming)));
            table.insert(TableEntry::String(TableAlignment::Right, format!("{}", e.amount.plain_amount().0)));
        }
        None => {
            table.insert(TableEntry::Empty);
            table.insert(TableEntry::Empty);
        }
    }
}

pub fn generate_complete_accounts_table(accounts: &Accounts, naming: &dyn AccountNaming) -> Table {
    let mut result : Table = Table::new();
    result.insert(TableEntry::String(TableAlignment::Left, "Account".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Debit".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Account".to_string()));
    result.insert(TableEntry::String(TableAlignment::Left, "Credit".to_string()));
    result.insert(TableEntry::NewRow);
    for ((date, ledger_id), entry_list) in accounts.iter() {
        result.insert(TableEntry::RowSeparator);
        let (debit_list, kredit_list) = split_debit_credit(entry_list);
        let mut debit_it = debit_list.iter();
        let mut credit_it = kredit_list.iter();
        while true {
            let debit = debit_it.next();
            let credit = credit_it.next();
            if(debit.is_none() && credit.is_none()) {
                break;
            }
            generate_side(&mut result, debit, naming);
            generate_side(&mut result, credit, naming);
            result.insert(TableEntry::NewRow);
        }
    }
    result
}