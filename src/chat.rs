use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::SlackApiError;
use crate::SlackClient;

/// Arguments for the chat.postMessage API method.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChatPostMessageArguments {
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name.
    pub channel: String,
    /// Text of the message to send. This field is usually required, unless you're providing only `attachments` or `blocks`.
    pub text: Option<String>,
    /// Blocks of the message to send. This field is usually required, unless you're providing only `text` or `attachments`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<serde_json::Value>>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<ChatPostMessageAttachment>>,
    /// Emoji to use as the icon for this message. Overrides icon_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Find and link user groups. No longer supports linking individual users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    /// JSON object with event_type and event_payload fields, presented as a URL-encoded string. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<serde_json::Value>>,
    /// Disable Slack markup parsing by setting to false. Enabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn: Option<bool>,
    /// Change how messages are treated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    /// Used in conjunction with thread_ts and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    /// Provide another message's ts value to make this message a reply. Avoid using a reply's ts value; use its parent instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// Set your bot's user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Attachment to a message.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChatPostMessageAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ChatPostMessageField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPostMessageField {
    pub title: String,
    pub value: String,
    pub short: bool,
}

pub trait Chat {
    /// Posts a message to a channel.
    fn post_message(&self, arguments: ChatPostMessageArguments) -> Result<String, SlackApiError>;
    /// Posts a message to a channel asynchronously.
    fn post_message_async(&self, arguments: ChatPostMessageArguments) -> Pin<Box<dyn Future<Output=Result<String, SlackApiError>> + Send + '_>>;
}

/// Implement the Chat trait for SlackClient.
impl Chat for SlackClient {
    fn post_message(&self, arguments: ChatPostMessageArguments) -> Result<String, SlackApiError> {
        self.runtime.block_on(self.post_message_async(arguments))
    }

    /// Posts a message to a channel.
    fn post_message_async(&self, arguments: ChatPostMessageArguments) -> Pin<Box<dyn Future<Output=Result<String, SlackApiError>> + Send + '_>> {

        // Check if the text, attachments, or blocks fields are provided
        if arguments.text.is_none() && arguments.attachments.is_none() && arguments.blocks.is_none() {
            return Box::pin(async { Err(SlackApiError::InvalidArgument("text, attachments, or blocks is required".into())) });
        }

        // Send the request to the Slack API
        Box::pin(async move {
            let res = self.client.post("https://slack.com/api/chat.postMessage")
                .bearer_auth(&self.token)
                .json(&arguments)
                .send()
                .await
                .map_err(SlackApiError::from)?
                .error_for_status()
                .map_err(SlackApiError::from)?;

            // Parse the response body as JSON
            let body: Value = res.json().await.map_err(SlackApiError::from)?;

            // Extract the message ID from the JSON
            let message_id = body["message"]["ts"].as_str().ok_or(SlackApiError::InvalidArgument("No message ID in response".into()))?.to_string();

            Ok(message_id)
        })
    }
}


#[cfg(test)]
mod chat_tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn chat_post_message() {
        let token = env::var("SLACK_TOKEN").expect("Expected a token in the environment");
        let channel_id = env::var("SLACK_CHANNEL_ID").expect("Expected a channel id in the environment");
        let text = "Hello, Slack from Rust!";

        let client = SlackClient::new(token.to_string());
        let arguments = ChatPostMessageArguments {
            channel: channel_id.to_string(),
            text: Option::from(text.to_string()),
            ..Default::default()
        };

        let result = client.post_message(arguments).await;
        assert!(result.is_ok(), "Failed to post message");
    }
}