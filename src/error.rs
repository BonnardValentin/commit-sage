use thiserror::Error;
use reqwest::StatusCode;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("API error: {}", .0.status().map_or("Network connection error. Please check your internet connection.", |s| match s {
        StatusCode::SERVICE_UNAVAILABLE => "Together.ai service is temporarily unavailable. Please try again in a few moments.",
        StatusCode::UNAUTHORIZED => "Invalid API key. Please check your Together.ai API key.",
        StatusCode::TOO_MANY_REQUESTS => "Rate limit exceeded. Please wait a moment before trying again.",
        _ => "Unexpected API error occurred.",
    }))]
    Request(#[from] reqwest::Error),

    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] toml::de::Error),

    #[error("No changes to commit. Make sure you have staged your changes with 'git add'")]
    NoChanges,

    #[error("API key not provided. Set TOGETHER_API_KEY environment variable or use --api-key")]
    NoApiKey,

    #[error("Failed to generate commit message: {0}")]
    CommitMessageGeneration(String),
}

pub type Result<T> = std::result::Result<T, Error>; 