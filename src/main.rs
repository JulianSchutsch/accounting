mod book;

use book::BookResult;

fn mainmain() -> BookResult {
    let settings = book::settings::Settings::read_from_file("/home/alexandrus/jsmjukvaruutveckling/booksettings.yaml".to_string())?;
    let exchange_rates = book::exchange_rate::import_using_settings(&settings)?;
    let bank_accounts = book::bankaccounts::import_using_settings(&settings)?;
    let ledger = book::ledger::import_using_settings(&settings)?;
    ledger.print();
    /*    let generator = book::swedish::Generator::new(&converter);
        let accounts = book::Generator::generate_accounts(&generator, &ledger).unwrap();
        accounts.print();
        let naming = book::swedish::AccountNaming::new();
        let complete_book = book::report::bookaccounts::complete::generate_complete_accounts_table(&accounts, &naming);
        complete_book.print();*/
    Ok(())
}

fn main() {
    let result = mainmain();
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
}
