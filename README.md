# Rust Slack

Slack API for Rust.

---

## Installation

Add `rustslack` as a dependency in your `Cargo.toml`:

```
[dependencies]
rustslack = { git = "https://github.com/nobuo-miura/rustslack.git", branch = "master" }
```

---

## Samples

### chat.postMessage

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

### chat.postMessage ( text only )

```rust
use rustslack::{SlackClient, Chat};

fn main() {
    // Slack Token
    let client = SlackClient::new("xoxb-*****************".to_string());

    // Channel ID & Text Message
    match client.post_message_text("**********".to_string(), "Hello, Slack!".to_string()) {
        Ok(response) => println!("Message sent successfully: {}", response),
        Err(e) => eprintln!("Error sending message: {:?}", e),
    }
}
```

### chat.delete

```rust
use rustslack::{SlackClient, Chat};

fn main() {
    // Slack Token
    let client = SlackClient::new("xoxb-*****************".to_string());

    // Channel ID & TS
    match client.delete("**********".to_string(), "*****.*****".to_string()) {
        Ok(_response) => println!("Message delete successfully"),
        Err(e) => eprintln!("Error delete message: {:?}", e),
    }
}
```



---

