# Veedy

**Veedy** is a brazing fast Telegram bot for Downloading videos from YouTube and other sites by typing a video title in inline mode.

## 🚀 Features

- **Inline Search**: Search YouTube videos directly from any chat by typing @YourBotName query.
- **Asynchronous**: Built on Tokio and Teloxide for non-blocking performance.
  
## How to use it

1. Types `@viidyBot cat videos`.
2. Choose the video you want and it will be downloaded onto your chat.

Works in groups, Saved messages and DMs.

## Want to set it up locally?

### 📋 Prerequisites

- Rust: (Latest Stable)
- Telegram Bot Token: From [@BotFather](https://t.me/BotFather).
- YouTube Data API Key: From Google Cloud Console.
- A Private Telegram Channel: For storing videos (Bot must be an Admin).

### ⚙️ Installation & Setup

1. Clone the repository

``` sh
    git clone https://github.com/aspects19/veedy.git
    cd veedy
```

2. Configure Environmental variables by adding each of your API keys and IDs.

```sh
    cp .env.example .env
```

3. Add your bot to the private channel specified in CHANNEL_ID and promote it to an **Adminstrator**
4. Run the bot.

``` sh
    cargo run
```

For a production environment run

```sh
cargo build --release
```

## 🚧 Roadmap & Planned Features

- [ ] Quality selection
- [ ] Remember user's prefered video quality
- [ ] Multi-platform support - support downloading links

### Have a suggestion? feel free to open an issue or submit a PR

## ⚖️ Disclaimer

This bot is for educational purposes only. Downloading content from YouTube may violate their Terms of Service. Please ensure you have the rights to download and distribute any content you access with this tool.
