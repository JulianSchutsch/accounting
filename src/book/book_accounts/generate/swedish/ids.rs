use crate::book::book_accounts::book_account_id::BookAccountId as Id;
use crate::book::*;

pub const CLAIMS_TO_CUSTOMERS:Id = Id(1510);
pub const CLAIMS_FROM_CUSTOMERS: Id = Id(2440);

pub const SALES_OF_SERVICES_WORLDWIDE:Id = Id(3305);
pub const SALES_OF_SERVICES_EU: Id = Id(3308);

pub const INCOMING_MOMS:Id = Id(2640);

pub const OUTGOING_MOMS: Id = Id(2610);
pub const OUTGOING_MOMS_REVERSE_CHARGE_25PERC: Id = Id(2614);
pub const INCOMING_MOMS_PROCUREMENT_ABROAD: Id = Id(2645);

pub const MEDIA_ADVERTISMENT: Id = Id(5970);
pub const SOFTWARE_LICENSES: Id = Id(5420);

pub const DEBT_TO_PRIVATE: Id = Id(2393);

pub const COMPANY_BANK_ACCOUNT: Id = Id(1930);

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
        Category::MediaAdvertisement => Ok(MEDIA_ADVERTISMENT),
        Category::SoftwareLicense => Ok(SOFTWARE_LICENSES),
        _ => Err(BookError::new(format!("Category {} not classified for invoice account", category)))
    }
}