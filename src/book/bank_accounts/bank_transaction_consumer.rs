use std::collections::HashSet;

use crate::book::*;

pub struct BankTransactionConsumer<'l> {
    consumed: HashSet<*const BankTransaction>,
    // The lifetime of above references must be constrained. Raw pointers do not have that functionality.
    marker: std::marker::PhantomData<&'l BankTransaction>
}

impl<'l> BankTransactionConsumer<'l> {
    pub fn is_consumed(&self, transaction: &'l BankTransaction) -> bool {
        let ptr: *const BankTransaction = transaction;
        self.consumed.get(&ptr).is_some()
    }

    pub fn try_consume(&mut self, transaction: &'l BankTransaction) -> bool {
        let ptr: *const BankTransaction = transaction;
        println!("Try Consume : {:?}", ptr);
        if let Some(_) = self.consumed.get(&ptr) {
            return false;
        }
        println!("Consume : {:?}", ptr);
        self.consumed.insert(ptr);
        true
    }

    pub fn new() -> Self {
        Self { consumed: HashSet::new(), marker: std::marker::PhantomData::default() }
    }
}