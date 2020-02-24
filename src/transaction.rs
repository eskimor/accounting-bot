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
pub mod message;
mod templates;

use posting::{*};
use message::Message;
use templates::Templates;

use std::fmt;
use chrono::naive::NaiveDate;


/**
 *   A very simply transaction type.
 *
 *   We only support transaction with exactly two postings, from and to. This is enough for this
 *   applications and provides stronger guarantees than a vector. (We know we have two postings,
 *   not one, not zero, but two and we need at least two postings for a valid transaction.
 */
#[derive(Copy,Clone,Debug)]
pub struct Transaction<S> {
    pub description: S,
    pub date: NaiveDate,
    /// Where the money comes from.
    pub from_posting: Posting<S>,
    /// Where the money goes to.
    pub to_posting: Posting<S>
}

impl<S> Transaction<S> {
    pub fn is_balanced(&self) -> bool {
        self.from_posting.amount + self.to_posting.amount == 0_u32.into()
    }
}

impl Transaction<String> {
    /// Parse a message directly into a `Transaction`.
    ///
    /// Templates will be looked up for filling out missing fields.
    pub fn parse_message(templates: &Templates, msg: &str) -> Self {
        Message::parse(msg).to_transaction(templates)
    }
}

impl From<&Transaction<&str>> for Transaction<String> {
    fn from(t: &Transaction<&str>) -> Self {
        Transaction {
            description: String::from(t.description),
            date: t.date,
            from_posting: Posting::from(&t.from_posting),
            to_posting: Posting::from(&t.to_posting),
        }
    }
}

impl<S: fmt::Display> fmt::Display for Transaction<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}   {}", self.date.format("%Y/%m/%d").to_string(), self.description)?;
        write!(f, "\n    {}", self.from_posting.render(FormatOption::WithAmount))?;
        write!(f, "\n    {}", self.to_posting.render(FormatOption::WithoutAmount))?;
        write!(f, "\n\n")
    }
}
