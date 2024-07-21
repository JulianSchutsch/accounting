use crate::book::*;

use super::ids::*;

pub fn generate_naming(accounts: &mut BookAccounts) {
    accounts.set_account_name(CLAIMS_TO_CUSTOMERS, "Claims to customers");
    accounts.set_account_name(INCOMING_MOMS, "Incoming moms");
    accounts.set_account_name(OUTGOING_MOMS_REVERSE_CHARGE_25PERC, "Outgoing moms, reverse charge, 25%");
    accounts.set_account_name(SALES_OF_SERVICES_WORLDWIDE, "Sales, Services, Worldwide");
    accounts.set_account_name(INCOMING_MOMS_PROCUREMENT_ABROAD, "Incoming moms, procurement abroad");
    accounts.set_account_name(CLAIMS_FROM_CUSTOMERS, "Claims from customers");
    accounts.set_account_name(MEDIA_ADVERTISEMENT, "Media advertisement");
    accounts.set_account_name(SOFTWARE_LICENSES, "Software licenses");
    accounts.set_account_name(DEBT_TO_COMPANY_OWNERS, "Debt to company owners");
    accounts.set_account_name(SERVICE_SALARY, "Service salary");
    accounts.set_account_name(SHORT_TERM_DEBT_SALARY, "Short term salary debt");
    accounts.set_account_name(EMPLOYEE_TAXES, "Employee preliminary taxes");
    accounts.set_account_name(EMPLOYER_SOCIAL_SECURITY_TAX, "Social security tax");
    accounts.set_account_name(EMPLOYER_SOCIAL_SECURITY_COSTS, "Social security costs");
    accounts.set_account_name(PRELIMINARY_PAID_COMPANY_TAX, "Preliminary paid company tax");
    accounts.set_account_name(SHORT_TERM_DEBT_TAXES, "Short term tax debt");
    accounts.set_account_name(COMPANY_BANK_ACCOUNT, "Company bank account");
    accounts.set_account_name(FINES, "Fines");
    accounts.set_account_name(TAX_FREE_INCOME, "Tax free income");
    accounts.set_account_name(MOMS_DEBT, "Moms debt");
    accounts.set_account_name(BOUND_CAPITAL, "Bound capital");
    accounts.set_account_name(SHARES_CAPITAL, "Shares capital");
    accounts.set_account_name(COMPANY_BANK_TRANSACTIONS, "Company transactions");
    accounts.set_account_name(COMPANY_CURRENCY_ACCOUNT, "Company currency account");
    accounts.set_account_name(INTEREST_FOR_CURRENT_ASSETS, "Interest for current assets");
    accounts.set_account_name(BANK_COSTS, "Bank costs");
    accounts.set_account_name(SHORT_TERM_DEBT_FROM_COMPANY_OWNERS, "Short term debt from company owners");
}
