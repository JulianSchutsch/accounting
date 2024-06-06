use std::string::ToString;
use book::*;

const ref1 : BankAccountReference = BankAccountReference::SwedishAccountNumber(bank_account_reference::SwedishAccountNumber{number: 0});
const ref2 : BankAccountReference = BankAccountReference::SwedishAccountNumber(bank_account_reference::SwedishAccountNumber{number: 1});
fn ref3() -> BankAccountReference { BankAccountReference::Name(bank_account_reference::Name{name: "Name".to_string()}) }

fn refs1() -> BankAccountReferences { BankAccountReferences::new_from_single(ref1) }
fn refs2() -> BankAccountReferences { BankAccountReferences::new_from_single(ref2) }
fn refs3() -> BankAccountReferences { BankAccountReferences::new_from_single(ref3()) }
fn refs12() -> BankAccountReferences { BankAccountReferences::new_from_iter(vec![ref1, ref2].iter()) }
fn refs13() -> BankAccountReferences { BankAccountReferences::new_from_iter(vec![ref1, ref3()].iter()) }

#[test]
fn test_refs_compare() {
    assert_eq!(refs1().matching(&refs1()), true);
    assert_eq!(refs1().matching(&refs2()), false);
    assert_eq!(refs1().matching(&refs13()), true);

    assert_eq!(refs12().matching(&refs1()), true);
    assert_eq!(refs12().matching(&refs2()), true);
    assert_eq!(refs12().matching(&refs13()), true);

    assert_eq!(refs13().matching(&refs1()), true);
    assert_eq!(refs13().matching(&refs2()), false);
    assert_eq!(refs13().matching(&refs3()), true);
}