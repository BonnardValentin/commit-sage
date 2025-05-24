use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Represents a message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Configuration for model generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub temperature: f32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
}

/// Context for model interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelContext {
    pub messages: Vec<Message>,
    pub config: GenerationConfig,
}

/// Trait for model providers (e.g., Together.ai, OpenAI, local models)
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// The error type returned by this provider
    type Error: std::error::Error + Send + Sync + 'static;

    /// Generate a response using the provided context
    async fn generate(&self, context: ModelContext) -> Result<String, Self::Error>;

    /// Get the model identifier
    fn model_id(&self) -> &str;

    /// Get the default configuration for this model
    fn default_config(&self) -> GenerationConfig;
}

/// Trait for commit message generators
#[async_trait]
pub trait CommitMessageGenerator: Send + Sync {
    /// The error type returned by this generator
    type Error: std::error::Error + Send + Sync + 'static;

    /// Generate a commit message from a diff
    async fn generate_message(&self, diff: &str) -> Result<String, Self::Error>;

    /// Validate a commit message format
    fn validate_message(&self, message: &str) -> bool;
}

/// Implementation of CommitMessageGenerator for any ModelProvider
#[async_trait]
impl<T: ModelProvider> CommitMessageGenerator for T {
    type Error = T::Error;

    async fn generate_message(&self, diff: &str) -> Result<String, Self::Error> {
        let context = ModelContext {
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a highly skilled developer who writes perfect conventional commit messages. \
                        You analyze git diffs and generate commit messages following the Conventional Commits specification. \
                        Your messages should be descriptive and precise, following this format:\n\
                        - For small changes: type(scope): concise description\n\
                        - For large changes (>5 files or >100 lines): type(scope): comprehensive description of main changes\n\
                        The type must be one of: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert.\n\
                        The scope should reflect the main component being changed.\n\
                        The description should be clear, precise, and written in imperative mood.\n\
                        For large changes, ensure the description captures the major components being modified.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!(
                        "Generate a conventional commit message for the following git diff.\n\
                        The message must follow the conventional commit format.\n\
                        If the diff is large (>5 files or >100 lines), make the description more comprehensive.\n\
                        Only return the commit message, nothing else.\n\n\
                        Diff:\n{}", 
                        diff
                    ),
                },
            ],
            config: self.default_config(),
        };

        self.generate(context).await
    }

    fn validate_message(&self, message: &str) -> bool {
        // Basic format: <type>[optional scope]: <description>
        let parts: Vec<&str> = message.splitn(2, ": ").collect();
        if parts.len() != 2 {
            return false;
        }

        let type_part = parts[0];
        let commit_type = if type_part.contains('(') {
            type_part.split('(').next().unwrap_or("")
        } else {
            type_part
        };

        let valid_types = [
            "feat", "fix", "docs", "style", "refactor",
            "perf", "test", "build", "ci", "chore", "revert"
        ];

        valid_types.contains(&commit_type)
    }
}

/// Together.ai implementation of ModelProvider
pub struct TogetherAiProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

#[async_trait]
impl ModelProvider for TogetherAiProvider {
    type Error = crate::Error;

    async fn generate(&self, context: ModelContext) -> Result<String, Self::Error> {
        let request = serde_json::json!({
            "model": self.model,
            "messages": context.messages,
            "temperature": context.config.temperature,
            "max_tokens": context.config.max_tokens,
            "stop": context.config.stop_sequences,
        });

        let response = self.client
            .post("https://api.together.xyz/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        response["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.trim().to_string())
            .ok_or_else(|| crate::Error::CommitMessageGeneration("No response from API".to_string()))
    }

    fn model_id(&self) -> &str {
        &self.model
    }

    fn default_config(&self) -> GenerationConfig {
        GenerationConfig {
            temperature: 0.3,
            max_tokens: 100,
            stop_sequences: vec!["\n".to_string()],
        }
    }
}

impl TogetherAiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
} 