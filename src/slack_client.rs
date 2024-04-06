use reqwest::Client;

/// Slack API client.
pub struct SlackClient {
    /// Slack API token.
    pub token: String,
    /// Reqwest client.
    pub client: Client,
}

/// Implementation of the Slack API client.
impl SlackClient {
    /// Create a new Slack API client.
    pub fn new(token: String) -> Self {
        SlackClient {
            token,
            client: Client::new(),
        }
    }
}