//! Here we get default values for values in our messages not provided out by the user.
use rust_decimal::Decimal;

/// A template can provide values for fields in a `Message` which are not mandatory.
#[derive(Copy, Clone, Debug)]
pub struct Template<'a> {
    pub amount: Option<Decimal>,
    pub from_account: Option<&'a str>,
    pub to_account: Option<&'a str>,
}

static DEFAULT_TEMPLATE : Template<'static> = Template {
    amount: None,
    from_account: Some("assets:cash"),
    to_account: Some("expenses:food"),
};


/// Lookup a template based on a given description.
///
/// Each word in the description will be looked up, the first successful (if any) lookup will
/// be returned.
pub fn lookup_description(desc: &str) -> Template {
    desc.split(' ').filter_map(|w| word_lookup(w)).next().unwrap_or(DEFAULT_TEMPLATE)
}

/// Lookup a template given a word in a description.
fn word_lookup (w: &str) -> Option<Template> {
    match w {
        "Mittagstisch" => Some(Template {
            amount: Some(Decimal::new(-350, 2)),
            ..DEFAULT_TEMPLATE
            }),
        "Billa" => Some(DEFAULT_TEMPLATE),
        "Spar"  => Some(Template {
            from_account: Some("assets:giro"),
            ..DEFAULT_TEMPLATE

        }),
        "Hofer"  => Some(Template {
            from_account: Some("assets:giro"),
            ..DEFAULT_TEMPLATE
        }),
        "Trinkgeld"  => Some(Template {
            to_account: Some("expenses:leisure"),
            ..DEFAULT_TEMPLATE
        }),
        _ => None
    }
}

