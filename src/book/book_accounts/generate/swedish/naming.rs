use crate::book::*;

use super::ids::*;

pub fn generate_naming(accounts: &mut BookAccounts) {
    accounts.set_account_name(CLAIMS_TO_CUSTOMERS, "Claims to customers");
    accounts.set_account_name(INCOMING_MOMS, "Incoming moms");
    accounts.set_account_name(OUTGOING_MOMS_REVERSE_CHARGE_25PERC, "Outgoing moms, reverse charge, 25%");
    accounts.set_account_name(SALES_OF_SERVICES_WORLDWIDE, "Sales, Services, Worldwide");
    accounts.set_account_name(INCOMING_MOMS_PROCUREMENT_ABROAD, "Incoming moms, procurement abroad");
    accounts.set_account_name(CLAIMS_FROM_CUSTOMERS, "Claims from customers");
    accounts.set_account_name(MEDIA_ADVERTISMENT, "Media advertisment");
    accounts.set_account_name(SOFTWARE_LICENSES, "Software licenses");
    accounts.set_account_name(DEBT_TO_PRIVATE, "Debt to private");
    accounts.set_account_name(SERVICE_SALARY, "Service salary");
    accounts.set_account_name(SHORT_TERM_DEBT_SALARY, "Shortterm salary debt");
    accounts.set_account_name(EMPLOYEE_PRELIMINARY_TAXES, "Employee preliminary taxes");
    accounts.set_account_name(EMPLOYER_COSTS, "Employers taxes");
    accounts.set_account_name(EMPLOYER_SOCIAL_FEES, "Employers social costs");
}
