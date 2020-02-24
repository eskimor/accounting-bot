//! Transaction message format accepted by the bot.
//!
//! For more details see the Readme.md.

#[cfg(test)]
mod tests;

use std::collections::{HashMap};
use chrono::offset;
use rust_decimal::Decimal;
use chrono::naive::NaiveDate;

use super::Transaction;
use super::posting::Posting;
use super::templates::Templates;


/// Transaction message as understood by our bot.
///
/// See Readme.md for details.
pub struct Message {
    /// Description as found in the message.
    description: String,
    /// The amount is optional, as the description will be looked up in known transaction and the
    /// amount can be picked up from there.
    amount: Option<Decimal>,
    /// Optional from_account, the default is assets:giro.
    from_account: Option<String>,
    /// Optional to_account, the default is expenses:food
    to_account: Option<String>,
}

impl Message {
    pub fn parse(msg: &str) -> Message {
        let mut words = msg.split(' ');
        let mut raw_description: Vec<&str> = Vec::new();
        let mut amount = None;
        for w in words.by_ref() {
            // We replace "," with "." to accept German numbers such as: 3,50 as well.
            match w.replace(",", ".").parse() {
                Err(_) => raw_description.push(w),
                Ok(v) => {
                    amount = Some(v);
                    break;
                }
            }
        }
        let from_account = words.next().map(|s| expand_account("assets", s));
        let to_account = words.next().map(|s| expand_account("expenses", s));
        Message {
            description: raw_description.join(" "),
            amount,
            from_account,
            to_account,
        }
    }

    /// Get a transaction for the given message.
    ///
    /// It will auto fill date with today and lookup the description in existing templates for
    /// filling out non specified fields.
    pub fn to_transaction(self, templates: &Templates) -> Transaction<String> {
        let def_template = Transaction {
            description: "",
            date: NaiveDate::from_ymd(1970, 1, 1),
            from_posting: Posting {
                account: "assets:cash",
                amount: Decimal::new(0, 2),
            },
            to_posting: Posting {
                account: "expenses:food",
                amount: Decimal::new(0, 2),
            }
        };
        let template_ref = templates.lookup_description(&self.description).unwrap_or(&def_template);
        let template = Transaction::from(template_ref);
        let now = offset::Local::today().naive_local();
        let mut result= Transaction {
            description: self.description,
            date: now,
            ..template
        };
        if let Some(msg_amount) = self.amount {
            result.from_posting.amount = -msg_amount;
            result.to_posting.amount = msg_amount;
        };
        if let Some(from_account) = self.from_account {
            result.from_posting.account = from_account;
        };
        if let Some(to_account) = self.to_account {
            result.to_posting.account = to_account;
        };
        result
    }

}

/// Prepends category to the raw account, if it is missing one.
///
/// E.g. parse_account("assets", "cash") will become "assets:cash".
fn expand_account(category: &str, raw: &str) -> String {
    if raw.contains(':') {
        String::from(raw)
    } else {
        String::from(category) + ":" + raw
    }
}
