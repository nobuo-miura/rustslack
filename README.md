# Rust Slack

Slack API for Rust.

---

## Installation

Add `rustslack` as a dependency in your `Cargo.toml`:

```
[dependencies]
rustslack = { git = "https://github.com/nobuo-miura/rustslack.git", branch = "master" }
```



## Sample

- chat.postMessage

```rust
use rustslack::{SlackClient, Chat, ChatPostMessageArguments};

fn main() {
    // Slack Token
    let client = SlackClient::new("xoxb-*****************".to_string());

    let arguments = ChatPostMessageArguments {
        // Channel ID
        channel: "**********".to_string(),
        text: Option::from("Hello, Slack!".to_string()),
        ..Default::default()
    };

    match client.post_message(arguments) {
        Ok(response) => println!("Message sent successfully: {}", response),
        Err(e) => eprintln!("Error sending message: {:?}", e),
    }
}
```

---

