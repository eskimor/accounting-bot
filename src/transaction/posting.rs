
//! A `Posting` as it occurres in `Transaction`s.

use std::fmt;
use rust_decimal::Decimal;

/// How to format a `Transaction`.
#[derive(Clone,Copy,Debug)]
pub enum FormatOption {
    /// Render with amount.
    WithAmount,
    /// Only render the account name.
    WithoutAmount
}

#[derive(Copy,Clone,Debug)]
pub struct Posting<S> {
    pub account: S,
    pub amount: Decimal,
}


impl<S: fmt::Display> Posting<S> {
    pub fn render(&self, opts: FormatOption) -> String {
        use FormatOption::{*};
        match opts {
            WithAmount => format!("{}      {}", self.account, self.amount),
            WithoutAmount => format!("{}", self.account)
        }
    }
}

impl<S: fmt::Display> fmt::Display for Posting<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render(FormatOption::WithAmount))
    }
}

impl From<&Posting<&str>> for Posting<String> {
    fn from(p: &Posting<&str>) -> Self {
        Posting {
            account: String::from(p.account),
            amount: p.amount,
        }
    }
}
