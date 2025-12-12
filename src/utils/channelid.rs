pub async fn channeid() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    dotenv().ok();
    
    let bot = Bot::from_env();
    
    let handler = Update::filter_channel_post()
        .endpoint(|msg: Message| async move {
            println!("Found Channel ID: {}", msg.chat.id);
            ResponseResult::<()>::Ok(())
        });

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}