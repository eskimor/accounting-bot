// extern crate accounting_bot;
use accounting_bot::cmds::transaction::{*};

use std::convert::TryFrom;
use std::fs::OpenOptions;
use std::io::Write;
// use std::sync::Arc;
use tbot::prelude::*;
use tbot::types::user::Id;
// Todo: Better error management - always report them to the user.
#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {
            // Auth check:
			if let Some(user) = &context.from {
				if user.id != Id(53228920) {
                   let resp = format!("Nice try {}, but computer says No!", user.first_name);
                   let call_result = context.send_message(&resp).call().await;
				   if let Err(err) = call_result {
                       dbg!(err);
                   }
                   return;
				}
			}
            let msg = &context.text.value;
			let resp = match Transaction::try_from(Message::parse(msg)) {
				Err(e) => format!("Error: {:?}", e),
				Ok(tr) => {
                    {
                        let mut ledger = OpenOptions::new().create(true).append(true).open("bot.ledger").expect("bot.ledger could not be opened!");
                        ledger.write(format!("{}", tr).as_bytes()).expect("bot.ledger coult not be written to!");
                    }// Make sure file is written, before sending success.
					format!("Added this to the ledger:\n\n{}", tr)
				}
			};
            let call_result = context.send_message(&resp).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.polling().start().await.unwrap();
}


// use telebot::Bot;
// use futures::stream::Stream;
// use std::env;
// use std::convert::TryFrom;

// import all available functions
// use telebot::functions::*;

// fn main() {
    // Create the bot
//     let mut bot = Bot::new(&env::var("TELEGRAM_BOT_KEY").unwrap()).update_interval(200);

    // Register a reply command which answers a message
//     let handle = bot.unknown_cmd()
//         .and_then(|(bot, msg)| {
//             match Transaction::try_from(Message::parse(&msg.text.unwrap())) {
//                 Err(e) => bot.message(msg.chat.id, format!("{:?}", e)).send(),
//                 Ok(r)  => bot.message(msg.chat.id, format!("\n{}", r)).send(),
//             }
//         })
//         .or_else(|bot,msg,err| bot.message(msg.chat.id, format!("Err: {:?}", err)))
//         .for_each(|_| Ok(()));

//     bot.run_with(handle);
// }
