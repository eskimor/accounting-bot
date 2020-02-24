//! This module defines the struct `Transaction` and `Message`. A `Message` is what the user
//! entered, the `Transaction` is an enriched `Message` suitable for putting on a ledger.

#[cfg(test)]
mod tests;
mod templates;

use std::fmt;
use std::convert::TryFrom;
use chrono::naive::NaiveDate;
use chrono::offset;
use rust_decimal::Decimal;


/// Transaction message as understood by our bot.
///
/// This type represents what the user can enter, see Readme.md for details.
///
/// By filling out missing information by use of `templates::Templates` This can be turned into a
/// proper `Transaction`.
///
#[derive(Clone,Debug)]
struct Message {
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

/// A `Message` with no missing fields. It is a very simply kind of a ledger `Transaction`.
pub struct Transaction {
    /// The date is not entered by the user, instead we just use the current date.
    date: NaiveDate,
    /// Description as found in the message.
    description: String,
    /// The amount that should be transferred from from_account. Positive values will reduce the
    /// balance of `from_account`.
    amount: Decimal,
    from_account: String,
    to_account: String,
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
}

/// What can go wrong when going from `Message` to `Transaction`?
#[derive(Copy, Clone, Debug)]
enum InvalidTransaction {
    AmountMissing,
    FromAccountMissing,
    ToAccountMissing
}

impl TryFrom<Message> for Transaction {

    type Error = InvalidTransaction;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        use InvalidTransaction::{*};
        let template = templates::lookup_description(&msg.description);
        let date = offset::Local::today().naive_local();
        let amount = msg.amount
            .or(template.amount)
            .map_or(Err(AmountMissing), Ok)?;
        let from_account = msg.from_account
            .or(template.from_account.map(String::from))
            .map_or(Err(FromAccountMissing), Ok)?;
        let to_account = msg.to_account
            .or(template.to_account.map(String::from))
            .map_or(Err(ToAccountMissing), Ok)?;
        Ok(Self {
            date,
            description: msg.description,
            amount,
            from_account,
            to_account
        })
    }
}


impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}   {}", self.date.format("%Y/%m/%d").to_string(), self.description)?;
        write!(f, "\n    {}      {}", self.from_account, -self.amount)?;
        write!(f, "\n    {}", self.to_account)?;
        write!(f, "\n\n")
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
