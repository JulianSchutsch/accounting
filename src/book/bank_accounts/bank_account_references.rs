use std::collections::HashSet;

use crate::book::*;

use super::bank_account_reference::BankAccountReference;

#[derive(Clone)]
pub struct BankAccountReferences {
    references: HashSet<BankAccountReference>,
}

impl BankAccountReferences {
    pub fn matching_transaction(&self, rhs: &BankTransactionReferences) -> bool {
        for left in self.references.iter() {
            match left {
                BankAccountReference::NamedAccount(name) => {
                    for right in rhs.references.iter() {
                        if name.name==*right {
                            return true;
                        }
                    }
                },
                _ => ()
            }
        }
        false
    }

    pub fn matching(&self, rhs: &Self) -> bool {
        !self.references.is_disjoint(&rhs.references)
    }

    pub fn contains(&self, rhs: &BankAccountReference) -> bool {
        self.references.contains(rhs)
    }

    pub fn extend(&mut self, references: BankAccountReferences) {
        self.references.extend(references.references.into_iter());
    }

    pub fn new_from_single(reference: BankAccountReference) -> Self {
        let mut references = HashSet::<BankAccountReference>::new();
        references.insert(reference);
        BankAccountReferences { references }
    }

    pub fn new_from_iter<'l, T: Iterator<Item=&'l BankAccountReference>>(iterable: T) -> BankAccountReferences {
        let references = iterable.map(|v| v.clone()).collect::<HashSet<BankAccountReference>>();
        BankAccountReferences { references }
    }
}

impl<'de> serde::Deserialize<'de> for BankAccountReferences {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<BankAccountReferences, D::Error> {
        let vec = Vec::<BankAccountReference>::deserialize(deserializer)?;
        let references= vec.into_iter().collect::<HashSet<BankAccountReference>>();
        Ok(BankAccountReferences{ references })
    }
}

impl std::fmt::Display for BankAccountReferences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for reference in self.references.iter() {
            write!(f, "{} ", reference)?;
        }
        write!(f, "]")
    }
}