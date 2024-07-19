use crate::book::*;

pub struct LedgerIdGenerator {
    fiscal_year: FiscalYearId,
    verification: SequenceGenerator<i32>,
    time: SequenceGenerator<i32>,
    transaction: SequenceGenerator<i32>
}

impl LedgerIdGenerator {
    pub fn new() -> LedgerIdGenerator {
        LedgerIdGenerator{ fiscal_year: FiscalYearId(0), verification: SequenceGenerator::new(), time: SequenceGenerator::new(), transaction: SequenceGenerator::new() }
    }

    pub fn select_fiscal_year(&mut self, period: Period) {
        self.fiscal_year = FiscalYearId(period.begin.id());
        self.verification.reset();
        self.time.reset();
        self.transaction.reset();
    }

    pub fn generate_verification_id(&mut self) -> LedgerId {
        LedgerId::new(self.fiscal_year, self.verification.generate(), LedgerIdKind::Verification)
    }

    pub fn generate_time_id(&mut self) -> LedgerId {
        LedgerId::new(self.fiscal_year, self.time.generate(), LedgerIdKind::Time)
    }

    pub fn generate_transaction_id(&mut self) -> LedgerId {
        LedgerId::new(self.fiscal_year, self.transaction.generate(), LedgerIdKind::Transaction)
    }
}
