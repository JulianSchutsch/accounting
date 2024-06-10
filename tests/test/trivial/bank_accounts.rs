use book::*;

pub fn bank_references() -> BankAccountReferences {
    BankAccountReferences::new_from_single(BankAccountReference::SwedishAccountNumber(SwedishAccountNumber{number: 1}))
}

pub fn bank_add(bank_accounts: &mut BankAccounts) -> BookResult {
    bank_accounts.add_account(bank_references(), BankAccountType::Account, Currency::SEK, Amount(0.0))
}