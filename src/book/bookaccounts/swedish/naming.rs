use crate::book::bookaccounts::accountid::AccountId;

use super::ids::*;

pub struct AccountNaming {

}

impl AccountNaming {
    pub fn new() -> Self {
        AccountNaming{}
    }
}

impl crate::book::bookaccounts::AccountNaming for AccountNaming {
    fn name(&self, id: AccountId) -> Option<String> {
        match id {
            CLAIMS_TO_CUSTOMERS => Some("Claims to Customers".to_string()),
            INCOMING_MOMS => Some("Incoming moms".to_string()),
            OUTGOING_MOMS_REVERSE_CHARGE_25PERC => Some("Outgoing moms, reverse charge, 25%".to_string()),
            SALES_OF_SERVICES_WORLDWIDE => Some("Sales, Services, Worldwide".to_string()),
            INCOMING_MOMS_PROCUREMENT_ABROAD => Some("Incoming moms, procurement abroad".to_string()),
            CLAIMS_FROM_CUSTOMERS => Some("Claims from customers".to_string()),
            _ => None,
        }
    }
}