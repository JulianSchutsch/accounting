use crate::book::*;

pub trait LedgerImporter {
    fn import(&self, ledger:&mut Ledger) -> BookResult;
}

pub struct MultiLedgerImporter {
    importers: Vec<Box<dyn LedgerImporter>>
}

impl MultiLedgerImporter {
    pub fn new(importers: Vec<Box<dyn LedgerImporter>>) -> Self {
        Self{ importers }
    }
}

impl LedgerImporter for MultiLedgerImporter {
    fn import(&self, ledger:&mut Ledger) -> BookResult {
        for importer in self.importers.iter() {
            importer.import(ledger)?;
        }
        Ok(())
    }
}