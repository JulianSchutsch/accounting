extern crate core;

mod book;

fn init_log() -> Result<(), log::SetLoggerError> {
    simplelog::TermLogger::init(
        log::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto
    )
}

fn main() {
    init_log().unwrap();
    let mut converter =  book::swedish::Converter::new(book::Currency::SEK);
    converter.add_riksbank_series("/home/alexandrus/Desktop/bokföring/SEKEUR.json".to_string(), book::Currency::EUR).unwrap();
    let mut generator = book::swedish::Generator::new();
    let mut ledger = book::Ledger::new();
    ledger.add_from_file("/home/alexandrus/jsmjukvaruutveckling/2023/data/02/data.yaml".to_string()).unwrap();
    ledger.add_from_file("/home/alexandrus/jsmjukvaruutveckling/2023/data/07/data.yaml".to_string()).unwrap();
    ledger.print();
    let accounts = book::Generator::generate_accounts(&generator, &converter, &ledger).unwrap();
    accounts.print();
    let naming = book::swedish::AccountNaming::new();
    let complete_book = book::report::bookaccounts::complete::generate_complete_accounts_table(&accounts, &naming);
    complete_book.print();
}
