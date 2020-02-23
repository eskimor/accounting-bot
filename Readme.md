# Accounting Bot

This is a telegram bot accepting messages in the form:

```
  Amount Description [from-account] [to-account]
```

Amount and description are mandatory, from-account and to-account have defaults
based on description. If there is no default for the given description,
from-account defaults to assets:cash and to-account defaults to expenses:other.

from-account can be abbreviated, e.g. assets:giro can be specified as just
giro. An abbreviated from-account will be expanded to assets:abbreviated and
to-account will be expanded to expenses:abbreviated.

# Structure

== bot.rs ==

This is the logic handling the telegram specifics, it implements the actual bot.

== transaction.rs ==

Implements struct Transaction, holding description, amounts and involved accounts. I has functionality for serializing to a ledger file.

== transaction/abbreviation_parser.rs ==

Parser for parsing explained above abbreviated format, it includes handling lookups for common descriptions.
