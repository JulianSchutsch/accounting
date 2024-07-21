use crate::book::book_accounts::book_account_id::BookAccountId as Id;
use crate::book::book_accounts::book_account_id_range::BookAccountIdRange as IdRange;
use crate::book::*;


pub const CLAIMS_TO_CUSTOMERS:Id = Id(1510);

pub const SHORT_TERM_DEBT_TAXES: Id = Id(1630);
pub const SHORT_TERM_DEBT_FROM_COMPANY_OWNERS: Id = Id(1685);

pub const CLAIMS_FROM_CUSTOMERS: Id = Id(2440);


pub const COMPANY_BANK_ACCOUNT: Id = Id(1930);
pub const COMPANY_BANK_TRANSACTIONS: Id = Id(1939);
pub const COMPANY_CURRENCY_ACCOUNT: Id = Id(1980);

pub const SALES_OF_SERVICES_WORLDWIDE:Id = Id(3305);
pub const SALES_OF_SERVICES_EU: Id = Id(3308);

pub const BOUND_CAPITAL: Id = Id(2080);
pub const SHARES_CAPITAL: Id = Id(2081);

pub const OUTGOING_MOMS: Id = Id(2610);
pub const OUTGOING_MOMS_REVERSE_CHARGE_25PERC: Id = Id(2614);
pub const INCOMING_MOMS_PROCUREMENT_ABROAD: Id = Id(2645);
pub const INCOMING_MOMS:Id = Id(2640);
pub const MOMS_RANGE: IdRange = IdRange::new(Id(2610), Id(2649));

pub const MOMS_DEBT: Id = Id(2650);

pub const MEDIA_ADVERTISEMENT: Id = Id(5970);
pub const SOFTWARE_LICENSES: Id = Id(5420);

pub const DEBT_TO_COMPANY_OWNERS: Id = Id(2393);

pub const SERVICE_SALARY: Id = Id(7210);
pub const SHORT_TERM_DEBT_SALARY: Id = Id(2821);

pub const EMPLOYEE_TAXES: Id = Id(2710);
pub const EMPLOYER_SOCIAL_SECURITY_TAX: Id = Id(2730);

pub const BANK_COSTS: Id = Id(6570);

pub const EMPLOYER_SOCIAL_SECURITY_COSTS: Id = Id(7510);

pub const PRELIMINARY_PAID_COMPANY_TAX:Id = Id(2518);

pub const INTEREST_FOR_CURRENT_ASSETS:Id = Id(8310);
pub const TAX_FREE_INCOME: Id = Id(8314);

pub const EXCHANGE_RATE_DIFFERENCES:Id = Id(8330);

pub const FINES: Id = Id(8423);

pub fn income_worldwide_account(category: Category) -> BookResult<Id> {
    match category {
        Category::Services => Ok(SALES_OF_SERVICES_WORLDWIDE),
        _ => Err(BookError::new("Unsupported worldwide income category"))
    }
}

pub fn invoice_moms(category: Category, reverse_charge: bool) -> BookResult<(Id, MomsFactor)> {
    if reverse_charge {
        Err(BookError::new("Unsupported case of moms"))
    } else {
        match category {
            Category::SoftwareLicense => Ok((INCOMING_MOMS, MomsFactor(0.25))),
            Category::MediaAdvertisement => Ok((INCOMING_MOMS, MomsFactor(0.25))),
            _ => Err(BookError::new("Unsupported case of moms"))
        }
    }
}

pub fn income_moms(category: Category, reverse_charge: bool) -> BookResult<(Id, MomsFactor)> {
    if reverse_charge {
        match category {
            Category::Services => Ok((OUTGOING_MOMS_REVERSE_CHARGE_25PERC, MomsFactor(0.25))),
            _ =>         Err(BookError::new("Unsupported case of moms"))
        }
    } else {
        Err(BookError::new("Unsupported case of moms"))
    }
}

pub fn invoice_account(category: Category) -> BookResult<Id> {
    match category {
        Category::MediaAdvertisement => Ok(MEDIA_ADVERTISEMENT),
        Category::SoftwareLicense => Ok(SOFTWARE_LICENSES),
        _ => Err(BookError::new(format!("Category {} not classified for invoice account", category)))
    }
}