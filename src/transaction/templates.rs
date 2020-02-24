use std::collections::{HashMap};
use chrono::naive::NaiveDate;
use rust_decimal::Decimal;
use crate::transaction::Transaction;
use crate::transaction::posting::Posting;

/// Lookup for words in a description to get a basic Transaction template.
pub struct Templates<'a>(HashMap<&'a str, Transaction<&'a str>>);

//
impl<'a> Templates<'a> {
    pub fn new() -> Templates<'static> {
        let raw = [("Mittagstisch", Transaction {
            description: "Mittagstisch",
            date: NaiveDate::from_ymd(1970, 1, 1),
            from_posting: Posting {
                account: "assets:cash",
                amount: Decimal::new(-350, 2),
            },
            to_posting: Posting {
                account: "expenses:food",
                amount: Decimal::new(350, 2),
            },
        }),
        ("Billa", Transaction {
            description: "Billa",
            date: NaiveDate::from_ymd(1970, 1, 1),
            from_posting: Posting {
                account: "assets:cash",
                amount: Decimal::new(0, 2),
            },
            to_posting: Posting {
                account: "expenses:food",
                amount: Decimal::new(0, 2),
            },
        }),
        ("Spar", Transaction {
            description: "Spar",
            date: NaiveDate::from_ymd(1970, 1, 1),
            from_posting: Posting {
                account: "assets:giro",
                amount: Decimal::new(0, 2),
            },
            to_posting: Posting {
                account: "expenses:food",
                amount: Decimal::new(0, 2),
            },
        }),
        ("Hofer", Transaction {
            description: "Hofer",
            date: NaiveDate::from_ymd(1970, 1, 1),
            from_posting: Posting {
                account: "assets:giro",
                amount: Decimal::new(0, 2),
            },
            to_posting: Posting {
                account: "expenses:food",
                amount: Decimal::new(0, 2),
            },
        }),
        ];
        let mut r = HashMap::new();
        r.reserve(raw.len());
        for (desc, trans) in raw.iter() {
            r.insert(*desc, *trans);
        }
        Templates(r)
    }

    /// Lookup a template based on a given description.
    ///
    /// Each word in the description will be looked up, the first successfull (if any) lookup will
    /// be returned.
    pub fn lookup_description(&self, desc: &str) -> Option<&Transaction<&str>> {
        desc.split(' ').filter_map(|w| self.0.get(w)).next()
        // desc.split(' ').filter_map(|w| self.0.get(w).map(|x| x.clone())).next()
    }
}

