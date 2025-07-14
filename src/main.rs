

use teloxide::prelude::*;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::RequestError;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};
use dotenvy::{dotenv, var};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();

    // Define the keyboard
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Like", "like"),
            InlineKeyboardButton::callback("Dislike", "dislike"),
        ],
    ]);

    // Handler for callback queries
    let callback_handler: Handler<'_, DependencyMap, Result<(), RequestError>, DpHandlerDescription> = Update::filter_callback_query().endpoint(
        |bot: Bot, q: CallbackQuery| async move {
            let response = match q.data.as_deref() {
                Some("like") => "Thanks for liking!",
                Some("dislike") => "Sorry you didn't like it!",
                _ => "Unknown",
            };

            if let Some(message) = q.message {
                match message {
                    MaybeInaccessibleMessage::Regular(msg) => {
                        bot.send_message(msg.chat.id, response).await?;
                    }
                    MaybeInaccessibleMessage::Inaccessible(_msg) => {
                        log::warn!("Message is inaccessible, cannot send response");
                    }
                }
            }
            bot.answer_callback_query(q.id).await?;
            Ok::<(), RequestError>(()) 
        },
    );

    // Send a message with the keyboard
let user_id = var("TEST_USER_ID").map_err(|_| "TEST_USER_ID environment variable not set").unwrap();
let user_id: i64 = user_id.parse().unwrap();
    bot.send_message(ChatId(user_id), "Do you like this bot?")
        .reply_markup(keyboard)
        .await
        .expect("Failed to send message");

    // Start the dispatcher
    Dispatcher::builder(bot, callback_handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}