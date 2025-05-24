use serde::{Deserialize, Serialize};
use crate::{Error, Result, AiConfig, is_conventional_commit};
use reqwest::StatusCode;
use std::{time::Duration};

const API_URL: &str = "https://api.together.xyz/v1/chat/completions";
const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY_MS: u64 = 1000;

#[derive(Debug)]
struct CommitContext {
    commit_type: String,
    file_types: Vec<String>,
    new_files: Vec<String>,
    modified_files: Vec<String>,
    total_additions: usize,
    total_deletions: usize,
}

impl CommitContext {
    fn from_diff(diff: &str) -> Self {
        let mut context = CommitContext {
            commit_type: String::new(),
            file_types: Vec::new(),
            new_files: Vec::new(),
            modified_files: Vec::new(),
            total_additions: 0,
            total_deletions: 0,
        };

        let mut current_file = String::new();
        for line in diff.lines() {
            if line.starts_with("diff --git") {
                current_file = line.split(' ').last().unwrap_or("").trim_start_matches('b').to_string();
                if let Some(ext) = current_file.split('.').last() {
                    context.file_types.push(ext.to_string());
                }
            } else if line.starts_with("new file") {
                context.new_files.push(current_file.clone());
            } else if line.starts_with("modified") {
                context.modified_files.push(current_file.clone());
            } else if line.starts_with('+') && !line.starts_with("+++") {
                context.total_additions += 1;
            } else if line.starts_with('-') && !line.starts_with("---") {
                context.total_deletions += 1;
            }
        }

        // Determine commit type based on context
        context.commit_type = if context.new_files.iter().any(|f| f.contains("Cargo.toml")) 
            && context.new_files.len() > 5 {
            "initial project setup".to_string()
        } else if context.file_types.iter().any(|t| t == "md" || t == "txt") 
            && context.file_types.len() == 1 {
            "documentation change".to_string()
        } else if context.new_files.iter().any(|f| f.contains("test") || f.contains("spec")) {
            "test addition".to_string()
        } else if context.total_additions > 100 || context.new_files.len() > 5 {
            "large feature implementation".to_string()
        } else if context.total_deletions > context.total_additions * 2 {
            "major refactoring".to_string()
        } else {
            "standard change".to_string()
        };

        context
    }

    fn get_suggested_type(&self) -> &'static str {
        match self.commit_type.as_str() {
            "initial project setup" => "feat",
            "documentation change" => "docs",
            "test addition" => "test",
            "large feature implementation" => "feat",
            "major refactoring" => "refactor",
            _ => "feat"
        }
    }

    fn to_prompt_context(&self) -> String {
        format!(
            "{} (suggested type: {}) with {} new files and {} modified files. \
            Changes include {} additions and {} deletions across file types: {}",
            self.commit_type,
            self.get_suggested_type(),
            self.new_files.len(),
            self.modified_files.len(),
            self.total_additions,
            self.total_deletions,
            self.file_types.join(", ")
        )
    }
}

#[derive(Debug, Serialize, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Clone)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
    stop: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    content: String,
}

pub struct AiClient {
    client: reqwest::Client,
    api_key: String,
    config: AiConfig,
}

impl AiClient {
    pub fn new(api_key: String, config: AiConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            config,
        }
    }

    pub async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        let context = CommitContext::from_diff(diff);
        
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: self.config.system_prompt.clone(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: self.config.user_prompt_template
                        .replace("{}", &context.to_prompt_context())
                        .replace("{}", diff),
                },
            ],
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
            stop: self.config.stop_sequences.clone(),
        };

        let mut last_error = None;
        for retry in 0..MAX_RETRIES {
            if retry > 0 {
                tokio::time::sleep(Duration::from_millis(
                    INITIAL_RETRY_DELAY_MS * (2_u64.pow(retry - 1))
                )).await;
            }

            match self.try_generate_message(&request).await {
                Ok(message) => {
                    // Pre-validate the message
                    if !is_conventional_commit(&message) {
                        continue; // Try again if format is invalid
                    }
                    // Validate the type matches the context
                    let msg_type = message.split(':').next().unwrap_or("")
                        .split('(').next().unwrap_or("");
                    if msg_type == context.get_suggested_type() {
                        return Ok(message);
                    }
                    // If we get here, the message is valid but doesn't match context
                    // Try again with a lower temperature
                    if retry < MAX_RETRIES - 1 {
                        let mut new_request = request.clone();
                        new_request.temperature *= 0.8;
                        if let Ok(new_message) = self.try_generate_message(&new_request).await {
                            if is_conventional_commit(&new_message) {
                                return Ok(new_message);
                            }
                        }
                    }
                    return Ok(message); // Use the original message if retries fail
                },
                Err(e) => {
                    if let Error::Request(ref req_err) = e {
                        if let Some(status) = req_err.status() {
                            if status == StatusCode::SERVICE_UNAVAILABLE 
                               || status == StatusCode::TOO_MANY_REQUESTS {
                                last_error = Some(e);
                                continue;
                            }
                        }
                    }
                    return Err(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| Error::CommitMessageGeneration(
            "Maximum retries exceeded".to_string()
        )))
    }

    async fn try_generate_message(&self, request: &ChatRequest) -> Result<String> {
        let response = self
            .client
            .post(API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?
            .error_for_status()?
            .json::<ChatResponse>()
            .await?;

        response
            .choices
            .first()
            .map(|choice| choice.message.content.trim().to_string())
            .ok_or_else(|| Error::CommitMessageGeneration("No response from API".to_string()))
    }
} 