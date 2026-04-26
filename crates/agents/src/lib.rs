use async_trait::async_trait;
use domain::{CoreError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// ...
#[derive(Debug, Clone, Serialize)]
pub struct Prompt {
    pub system_prompt: String,
    pub user_prompt: String,
}

/// ...
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &Prompt) -> Result<String>;
}

/// ...
#[derive(Serialize)]
struct LlamaCppRequest {
    prompt: String,
    n_predict: i32,
    temperature: f32,
}

/// ...
#[derive(Deserialize)]
struct LlamaCppResponse {
    content: String,
}

/// ...
pub struct LlamaCppProvider {
    client: Client,
    server_url: String,
}

impl LlamaCppProvider {
    pub fn new(server_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            server_url: server_url.into(),
        }
    }
}

#[async_trait]
impl LlmProvider for LlamaCppProvider {
    async fn generate(&self, prompt: &Prompt) -> Result<String> {
        let full_prompt = format!(
            "<|system|>\n{}<|user|>\n{}<|assistant|>\n",
            prompt.system_prompt, prompt.user_prompt
        );

        let req_body = LlamaCppRequest {
            prompt: full_prompt,
            n_predict: 2048,
            temperature: 0.2, // typically want low temp for deterministic agent codegen
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

/// ...
pub struct BaseAgent {
    pub name: String,
    pub provider: Box<dyn LlmProvider>,
}

impl BaseAgent {
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
