use super::swedish;

pub enum AnnualAccounts {
    SwedishK2(swedish::K2)
}

impl AnnualAccounts {
    pub fn print(&self) {
        match self {
            AnnualAccounts::SwedishK2(e) => e.print()
        }
    }
}