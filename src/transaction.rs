//! This module defines the struct `Transaction` which represents a transaction happening on a
//! ledger. The definition is oriented on what is actually needed to represent transaction as used
//! by this bot, it is not possible to represent all possible transactions supported by
//! hledger. E.g. multiple currencies are not supported.
//!
//! In addition to the type itself it introduces functionality for writing out the transactions to
//! a ledger file. It does not care about parsing.

#[cfg(test)]
mod tests;

use std::fmt;

use rust_decimal::Decimal;
use chrono::naive::NaiveDate;


struct Transaction {
    description: String,
    date: NaiveDate,
    postings: Vec<Posting>
}

struct Posting {
    account: String,
    amount: Decimal,
}

impl Transaction {
    fn is_balanced(&self) -> bool {
        self.postings.iter().map(|p| p.amount).sum::<Decimal>() == 0_i32.into()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}   {}", self.date.format("%Y/%m/%d").to_string(), self.description)?;
        for p in &self.postings {
            write!(f, "\n    {}", p)?
        }
        write!(f, "\n\n")
    }
}

impl fmt::Display for Posting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}      {}", self.account, self.amount)
    }
}
