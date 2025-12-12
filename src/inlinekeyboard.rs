use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[allow(unused)]
pub async fn keyboardbtns() -> InlineKeyboardMarkup {
       let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("4K", "4K"),
            InlineKeyboardButton::callback("1080P", "1080P"),
            InlineKeyboardButton::callback("720P", "720P"),
        ],
        vec![
            InlineKeyboardButton::callback("480P", "480P"),
            InlineKeyboardButton::callback("360P", "360P"),
            InlineKeyboardButton::callback("256K", "256K"),
            InlineKeyboardButton::callback("128K", "128K"),
        ]
    ]);
    
    keyboard
}