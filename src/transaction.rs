//! This module defines the struct `Transaction` which represents a transaction happening on a
//! ledger. The definition is oriented on what is actually needed to represent transaction as used
//! by this bot, it is not possible to represent all possible transactions supported by
//! hledger. E.g. multiple currencies are not supported.
//!
//! In addition to the type itself it introduces functionality for writing out the transactions to
//! a ledger file. It does not care about parsing.

#[cfg(test)]
mod tests;
pub mod posting;

use posting::{*};

use std::fmt;
use chrono::naive::NaiveDate;


/**
 *   A very simply transaction type.
 *
 *   We only support transaction with exactly two postings, from and to. This is enough for this
 *   applications and provides stronger guarantees than a vector. (We know we have two postings,
 *   not one, not zero, but two and we need at least two postings for a valid transaction.
 */
struct Transaction {
    description: String,
    date: NaiveDate,
    /// Where the money comes from.
    from_posting: Posting,
    /// Where the money goes to.
    to_posting: Posting
}


impl Transaction {
    pub fn is_balanced(&self) -> bool {
        self.from_posting.amount + self.to_posting.amount == 0_u32.into()
    }
}


impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}   {}", self.date.format("%Y/%m/%d").to_string(), self.description)?;
        write!(f, "\n    {}", self.from_posting.render(FormatOption::WithAmount))?;
        write!(f, "\n    {}", self.to_posting.render(FormatOption::WithoutAmount))?;
        write!(f, "\n\n")
    }
}
