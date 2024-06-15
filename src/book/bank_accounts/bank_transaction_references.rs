use std::collections::HashSet;
use serde::Deserializer;

#[derive(Debug)]
pub struct BankTransactionReferences {
    references: HashSet<String>
}

impl BankTransactionReferences {

    pub fn description(&self) -> String {
        self.references.iter().fold(String::new(), |acc, v| {
            if acc.is_empty() { v.clone() } else { acc + ";" + v }
        })
    }

    pub fn is_match(&self, other: &BankTransactionReferences) -> bool {
        other.references.is_subset(&self.references)
    }

    pub fn new_from_single(s: &str) -> Self {
        let mut references: HashSet<String> = HashSet::new();
        references.insert(s.to_string());
        Self { references }
    }

}

impl<'l> serde::Deserialize<'l> for BankTransactionReferences {
    fn deserialize<D: Deserializer<'l>>(deserializer: D) -> Result<Self, D::Error> {
        let references: HashSet<String>= (Vec::<String>::deserialize(deserializer)?).into_iter().collect();
        Ok(BankTransactionReferences{ references })
    }

}