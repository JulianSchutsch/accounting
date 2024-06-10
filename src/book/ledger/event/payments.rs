use std::collections::BTreeMap;
use crate::book::*;

type PaymentVec = Vec<Payment>;
type PaymentVecIter<'l> = std::slice::Iter<'l, Payment>;
type PaymentMap = BTreeMap<Date, PaymentVec>;
type PaymentMapIter<'l> = std::collections::btree_map::Iter<'l, Date, PaymentVec>;

#[derive(Debug, Default)]
pub struct Payments {
    payments: PaymentMap,
}

pub struct PaymentsDateFilter<'l> {
    date: Date,
    inverse: bool,
    top_iter: PaymentMapIter<'l>,
    bottom_iter: PaymentVecIter<'l>
}

impl PaymentsDateFilter<'_> {
    fn date_match(&self, date: Date) -> bool {
        if !self.inverse {
            self.date == date
        } else {
            self.date != date
        }
    }
}

impl<'l> Iterator for PaymentsDateFilter<'l> {
    type Item = &'l Payment;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some(entry) = self.bottom_iter.next() {
                if self.date_match(entry.date) {
                    return Some(entry);
                }
            }
            let Some((_, next_bottom)) = self.top_iter.next() else {
                return None
            };
            self.bottom_iter = next_bottom.iter();
        }
    }
}

impl Payments {
    pub fn iter_date(&self, date: Date) -> PaymentsDateFilter {
        PaymentsDateFilter{ date, inverse: false, top_iter: self.payments.iter(), bottom_iter: PaymentVecIter::default() }
    }

    pub fn iter_inverse_date(&self, date: Date) -> PaymentsDateFilter {
        PaymentsDateFilter{ date, inverse: false, top_iter: self.payments.iter(), bottom_iter: PaymentVecIter::default() }
    }

    pub fn payed_on_date(&self, date: Date) -> Amount {
        self.iter_date(date).fold(Amount(0.09), |acc, v| if date==v.date { acc + v.amount } else { acc })
    }

    pub fn payed_not_on_date(&self, date: Date) -> Amount {
        self.iter_date(date).fold(Amount(0.0), |acc, v| if date!=v.date { acc + v.amount } else { acc })
    }

    pub fn add(&mut self, e: Payment) {
        self.payments.entry(e.date).or_insert(vec![]).push(e);
    }
}