use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

use super::base::{Provider, ProviderUsage, Usage};
use super::configs::ModelConfig;
use super::errors::ProviderError;
use super::formats::openai::{
    create_request, get_usage, is_context_length_error, response_to_message,
};
use super::oauth;
use super::utils::{get_model, handle_response, ImageFormat};
use crate::message::Message;
use mcp_core::tool::Tool;

const DEFAULT_CLIENT_ID: &str = "databricks-cli";
const DEFAULT_REDIRECT_URL: &str = "http://localhost:8020";
const DEFAULT_SCOPES: &[&str] = &["all-apis"];
pub const DATABRICKS_DEFAULT_MODEL: &str = "claude-3-5-sonnet-2";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabricksAuth {
    Token(String),
    OAuth {
        host: String,
        client_id: String,
        redirect_url: String,
        scopes: Vec<String>,
    },
}

impl DatabricksAuth {
    /// Create a new OAuth configuration with default values
    pub fn oauth(host: String) -> Self {
        Self::OAuth {
            host,
            client_id: DEFAULT_CLIENT_ID.to_string(),
            redirect_url: DEFAULT_REDIRECT_URL.to_string(),
            scopes: DEFAULT_SCOPES.iter().map(|s| s.to_string()).collect(),
        }
    }
    pub fn token(token: String) -> Self {
        Self::Token(token)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DatabricksProvider {
    #[serde(skip)]
    client: Client,
    host: String,
    auth: DatabricksAuth,
    model: ModelConfig,
    image_format: ImageFormat,
}

impl DatabricksProvider {
    pub fn from_env() -> Result<Self> {
        // Although we don't need host to be stored secretly, we use the keyring to make
        // it easier to coordinate with configuration. We could consider a non secret storage tool
        // elsewhere in the future
        let host = crate::key_manager::get_keyring_secret("DATABRICKS_HOST", Default::default())?;
        let model_name = std::env::var("DATABRICKS_MODEL")
            .unwrap_or_else(|_| DATABRICKS_DEFAULT_MODEL.to_string());

        let client = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        // If we find a databricks token we prefer that
        if let Ok(api_key) =
            crate::key_manager::get_keyring_secret("DATABRICKS_TOKEN", Default::default())
        {
            return Ok(Self {
                client,
                host: host.clone(),
                auth: DatabricksAuth::token(api_key),
                model: ModelConfig::new(model_name),
                image_format: ImageFormat::Anthropic,
            });
        }

        // Otherwise use Oauth flow
        Ok(Self {
            client,
            host: host.clone(),
            auth: DatabricksAuth::oauth(host),
            model: ModelConfig::new(model_name),
            image_format: ImageFormat::Anthropic,
        })
    }

    async fn ensure_auth_header(&self) -> Result<String> {
        match &self.auth {
            DatabricksAuth::Token(token) => Ok(format!("Bearer {}", token)),
            DatabricksAuth::OAuth {
                host,
                client_id,
                redirect_url,
                scopes,
            } => {
                let token =
                    oauth::get_oauth_token_async(host, client_id, redirect_url, scopes).await?;
                Ok(format!("Bearer {}", token))
            }
        }
    }

    async fn post(&self, payload: Value) -> Result<Value, ProviderError> {
        let url = format!(
            "{}/serving-endpoints/{}/invocations",
            self.host.trim_end_matches('/'),
            self.model.model_name
        );

        let auth_header = self.ensure_auth_header().await.unwrap();
        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&payload)
            .send()
            .await
            .unwrap();

        handle_response(payload, response).await
    }
}

#[async_trait]
impl Provider for DatabricksProvider {
    fn get_model_config(&self) -> &ModelConfig {
        &self.model
    }

    #[tracing::instrument(
        skip(self, system, messages, tools),
        fields(model_config, input, output, input_tokens, output_tokens, total_tokens)
    )]
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        let mut payload =
            create_request(&self.model, system, messages, tools, &self.image_format).unwrap();
        // Remove the model key which is part of the url with databricks
        payload
            .as_object_mut()
            .expect("payload should have model key")
            .remove("model");

        let response = self.post(payload.clone()).await?;

        // Raise specific error if context length is exceeded
        if let Some(error) = response.get("error") {
            if let Some(err) = is_context_length_error(error) {
                return Err(ProviderError::ContextLengthExceeded(err.to_string()));
            }
            return Err(ProviderError::RequestFailed(error.to_string()));
        }

        // Parse response
        let message = response_to_message(response.clone()).unwrap();
        let usage = self.get_usage(&response).unwrap();
        let model = get_model(&response);
        super::utils::emit_debug_trace(self, &payload, &response, &usage);

        Ok((message, ProviderUsage::new(model, usage)))
    }

    fn get_usage(&self, data: &Value) -> Result<Usage> {
        get_usage(data)
    }
}
