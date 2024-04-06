pub use chat::{Chat, ChatPostMessageArguments, ChatPostMessageAttachment};
pub use slack_client::SlackClient;

mod slack_client;
mod chat;
mod errors;

