use std::sync::Arc;

use reqwest::Client;
use tokio::runtime::Runtime;

/// Slack API client.
pub struct SlackClient {
    /// Slack API token.
    pub token: String,
    /// Reqwest client.
    pub client: Client,
    /// Tokio runtime.
    pub runtime: Arc<Runtime>,
}

/// Implementation of the Slack API client.
impl SlackClient {
    /// Create a new Slack API client.
    pub fn new(token: String) -> Self {
        let runtime = Arc::new(Runtime::new().unwrap());

        SlackClient {
            token,
            client: Client::new(),
            runtime,
        }
    }
}