use crate::book::*;

use super::ids;
use super::active_associables::*;
use super::params::*;

#[derive(Debug)]
struct AssociableExchange {
    ledger_id: LedgerId,
    exchange: Exchange
}

struct Dir {
    pub book_amount: Amount,
    pub realised_book_amount: Amount
}

fn calculate_exchange_direction(a: &Exchange, b: &Exchange, p: &Params ) -> BookResult<Dir> {
    if a.currency==b.currency {
        return Err(BookError::new("Exchange between same currencies is unnecessary"));
    }
    if !(a.currency==p.first.exchange_rates.book_currency) && !(b.currency==p.first.exchange_rates.book_currency) {
        return Err(BookError::new("None of the currencies in this exchange is a book currency"));
    }
    println!("Exchange amounts: {} {} {} {}", a.amount, b.amount, a.currency, b.currency);
    if a.currency==p.first.exchange_rates.book_currency {
        Ok(Dir{
            book_amount:p.first.exchange_rates.convert_into_book_currency(b.date, b.currency, b.amount, None)?,
            realised_book_amount: a.amount
        })
    } else {
        Ok(Dir {
            book_amount: p.first.exchange_rates.convert_into_book_currency(a.date, a.currency, a.amount, None)?,
            realised_book_amount: b.amount
        })
    }
}

impl Associable<Exchange, Params<'_>> for AssociableExchange {
    fn associate(&mut self, _ledger_id: LedgerId, event: &Exchange, p: &mut Params) -> BookResult<AssociableChange> {
        // TODO: Date may not be the best way to associate
        if self.exchange.date==event.date {
            let dir = calculate_exchange_direction(&self.exchange, event, p)?;
            let currency_change = BookAmount::from_signed_amount(dir.book_amount);
            let book_change = BookAmount::from_signed_amount(dir.realised_book_amount);
            let correction = BookAmount::from_signed_amount(dir.realised_book_amount+dir.book_amount).invert();
            p.book.add_entry(self.ledger_id, self.exchange.date, &self.exchange.id, ids::COMPANY_CURRENCY_ACCOUNT, currency_change);
            p.book.add_entry(self.ledger_id, self.exchange.date, &self.exchange.id, ids::COMPANY_BANK_ACCOUNT, book_change);
            p.book.add_entry(self.ledger_id, self.exchange.date, &self.exchange.id, ids::EXCHANGE_RATE_DIFFERENCES, correction);
            return Ok(AssociableChange::Close);
        }
        Ok(AssociableChange::NoMatch)
    }
    fn describe(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn add<'p>(ledger_id: LedgerId, event: &Exchange, p: &mut Params<'p>, associables: &mut ActiveAssociables<'p>) -> BookResult {
    if !associables.exchanges.associate(ledger_id, event, p)? {
        associables.exchanges.register(Box::new(AssociableExchange {
            ledger_id: ledger_id,
            exchange: event.clone(),
        }));
    }
    Ok(())
}