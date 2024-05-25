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
    let mut ledger = book::Ledger::new();
    ledger.add_from_file("/home/alexandrus/jsmjukvaruutveckling/2023/data/02/data.yaml".to_string()).unwrap();
    ledger.print();
    let accounts = book::Generator::generate_accounts(&book::swedish::Generator, &ledger).unwrap();
    accounts.print();
}
