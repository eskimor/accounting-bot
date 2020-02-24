# Accounting Bot

This is a telegram bot accepting messages in the form:

```
  Description [Amount] [from-account] [to-account]
```

Description is mandatory, amount, from-account and to-account have defaults
based on description. If there is no default for the given description,
from-account defaults to assets:cash and to-account defaults to expenses:food.
Amount defaults to zero, which is pointless obviously.

from-account can be abbreviated, e.g. assets:giro can be specified as just
giro. An abbreviated from-account will be expanded to assets:abbreviated and
to-account will be expanded to expenses:abbreviated.

The optional fields must be present if some later field needs to be specified. E.g. you have to specify `amount` if you want to be able to set `from-account`.


Notes about amount: As the most common usage is for amount to be negative (I spend money), negative values will be assumed by the bot. E.g. if the user specifies 3.5 it actually means -3.5. Therefore if we actually gain money, the user needs to specify a negative amount in the message.

# Structure

== bot.rs ==

This is the logic handling the telegram specifics, it implements the actual bot.


== cmds/transaction.rs ==

Implements the command to create transactions. This is the only command implemented right now and adheres to the format specified above. In the future this means, any command that does not start with a recognized keword will be interpreted as a transaction to be added.

