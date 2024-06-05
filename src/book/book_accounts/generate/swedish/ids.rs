use crate::book::book_accounts::book_account_id::BookAccountId as Id;

pub const CLAIMS_TO_CUSTOMERS:Id = Id(1510);
pub const CLAIMS_FROM_CUSTOMERS: Id = Id(2440);

pub const SALES_OF_SERVICES_WORLDWIDE:Id = Id(3305);
pub const SALES_OF_SERVICES_EU: Id = Id(3308);

pub const INCOMING_MOMS:Id = Id(2640);

pub const OUTGOING_MOMS: Id = Id(2610);
pub const OUTGOING_MOMS_REVERSE_CHARGE_25PERC: Id = Id(2614);
pub const INCOMING_MOMS_PROCUREMENT_ABROAD: Id = Id(2645);