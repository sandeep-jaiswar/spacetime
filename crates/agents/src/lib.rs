use async_trait::async_trait;
use domain::{CoreError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// The prompt structure to send to an LLM provider
#[derive(Debug, Clone, Serialize)]
pub struct Prompt {
    pub system_prompt: String,
    pub user_prompt: String,
}

/// The standard interface for any AI model backends driving agents
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &Prompt) -> Result<String>;
}

/// Request structure mirroring the llama.cpp `/completion` HTTP API
#[derive(Serialize)]
struct LlamaCppRequest {
    prompt: String,
    n_predict: i32,
    temperature: f32,
}

/// Response payload from expanding a llama.cpp node
#[derive(Deserialize)]
struct LlamaCppResponse {
    content: String,
}

/// Provider managing communication with a locally hosted llama.cpp engine
pub struct LlamaCppProvider {
    client: Client,
    server_url: String,
    chat_template: String,
    max_tokens: i32,
    temperature: f32,
}

impl LlamaCppProvider {
    pub fn new(server_url: impl Into<String>) -> Self {
        // Updated: Using Client builder with timeouts
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            server_url: server_url.into(),
            chat_template: "<|system|>\n{}<|user|>\n{}<|assistant|>\n".to_string(),
            max_tokens: 2048,
            temperature: 0.2,
        }
    }

    pub fn with_chat_template(mut self, template: impl Into<String>) -> Self {
        self.chat_template = template.into();
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
}

#[async_trait]
impl LlmProvider for LlamaCppProvider {
    async fn generate(&self, prompt: &Prompt) -> Result<String> {
        let full_prompt = self.chat_template
            .split("{}")
            .zip([&prompt.system_prompt, &prompt.user_prompt].into_iter().chain(std::iter::repeat(&"".to_string())))
            .map(|(part, val)| format!("{}{}", part, val))
            .collect::<String>();

        let req_body = LlamaCppRequest {
            prompt: full_prompt,
            n_predict: self.max_tokens,
            temperature: self.temperature,
        };

        let url = format!("{}/completion", self.server_url);

        let res = self
            .client
            .post(&url)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| {
                CoreError::OrchestrationError(format!("LLM HTTP Request failed: {}", e))
            })?;

        if !res.status().is_success() {
            return Err(CoreError::OrchestrationError(format!(
                "llama.cpp server returned error status: {}",
                res.status()
            )));
        }

        let resp_payload: LlamaCppResponse = res
            .json()
            .await
            .map_err(|e| CoreError::OrchestrationError(format!("Failed to parse JSON: {}", e)))?;

        Ok(resp_payload.content.trim().to_string())
    }
}

/// An autonomous agent struct wrapper utilizing an LLM Provider
#[derive(Clone)]
pub struct BaseAgent {
    name: String,
    provider: Arc<dyn LlmProvider>,
}

impl BaseAgent {
    pub fn new(name: impl Into<String>, provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            name: name.into(),
            provider,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn execute_task(&self, task_description: &str) -> Result<String> {
        let prompt = Prompt {
            system_prompt: format!(
                "You are an autonomous engineering agent named {}. Execute the tasks accurately.",
                self.name
            ),
            user_prompt: task_description.to_string(),
        };

        self.provider.generate(&prompt).await
    }
}
