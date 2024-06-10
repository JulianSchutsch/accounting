use book::*;
use crate::test::trivial;

#[test]
fn simple_main_book() {
    let mut import = trivial::import();
    let mut ledger_id = trivial::ledger_id();
    let income = trivial::world_income("A".to_string(), Date::from_str("2000-06-01").unwrap(), Amount(1000.0), Currency::SEK, Category::Services, Country::Switzerland);
    import.ledger.events.insert(ledger_id, income);
    let book_accounts = book_accounts::generate(&import).unwrap();
    book_accounts::verify::balance::verify(&book_accounts).unwrap();
}