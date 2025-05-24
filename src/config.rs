use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ai: AiConfig,
    pub git: GitConfig,
    pub commit: CommitConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiConfig {
    /// The AI model to use
    pub model: String,
    /// Temperature for model output (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum tokens in the response
    pub max_tokens: u32,
    /// Stop sequences for the model
    pub stop_sequences: Vec<String>,
    /// System prompt for the AI
    pub system_prompt: String,
    /// User prompt template
    pub user_prompt_template: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitConfig {
    /// Path to the git repository
    pub repo_path: PathBuf,
    /// Whether to include untracked files in the diff
    pub include_untracked: bool,
    /// Whether to show the diff before generating commit message
    pub show_diff: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitConfig {
    /// List of allowed commit types
    pub allowed_types: Vec<String>,
    /// Maximum length of commit message
    pub max_length: usize,
    /// Whether to automatically commit after generating message
    pub auto_commit: bool,
    /// Whether to verify commit message format
    pub verify_format: bool,
    /// Whether to require user confirmation before committing
    pub require_confirmation: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            git: GitConfig::default(),
            commit: CommitConfig::default(),
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            model: "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string(),
            temperature: 0.3,
            max_tokens: 100,
            stop_sequences: vec!["\n".to_string()],
            system_prompt: "You are a highly skilled developer who writes perfect conventional commit messages. \
                Your task is to analyze git diffs and generate commit messages that strictly follow the Conventional Commits specification.\n\n\
                COMMIT FORMAT RULES:\n\
                1. Messages MUST follow this exact structure: type(scope): description\n\
                2. Valid types are: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert\n\
                3. Scope should be the main component being changed (e.g., auth, api, core)\n\
                4. Description must:\n\
                   - Start with a lowercase letter\n\
                   - Use imperative mood (e.g., 'add' not 'adds')\n\
                   - No period at the end\n\
                   - Stay under 72 characters total\n\n\
                EXAMPLES BY CHANGE TYPE:\n\
                1. Initial Project Setup:\n\
                   ✓ feat(core): implement AI commit generator with Together.ai and async traits\n\
                   ✓ feat(arch): establish modular design with CLI and configuration system\n\
                   ✗ feat(project): initialize repository with basic files\n\
                   ✗ chore: initial commit\n\n\
                2. Architecture and Core Features:\n\
                   ✓ feat(arch): add provider-based design with async trait system\n\
                   ✓ feat(core): integrate AI with retry logic and error handling\n\
                   ✗ feat: add new features\n\n\
                3. CLI and Configuration:\n\
                   ✓ feat(cli): add command-line interface with comprehensive options\n\
                   ✓ feat(config): implement TOML-based configuration system\n\
                   ✗ feat: add CLI tool\n\n\
                INITIAL COMMIT REQUIREMENTS:\n\
                1. Must be concise but informative (under 72 chars):\n\
                   - Focus on 1-2 key architectural patterns\n\
                   - Mention primary integration\n\
                   - Highlight main feature\n\
                2. Must highlight unique aspects:\n\
                   - AI provider integration\n\
                   - Error handling approach\n\
                   - Configuration system\n\
                3. Use appropriate scope:\n\
                   - 'core' for fundamental features\n\
                   - 'arch' for architectural decisions\n\
                   - 'project' only for basic setup\n\
                4. Be focused and precise:\n\
                   - Choose most important components\n\
                   - Prioritize key technologies\n\
                   - Select defining patterns".to_string(),
            user_prompt_template: "Generate a conventional commit message for the following git diff.\n\
                The message MUST strictly follow the conventional commit format rules specified above.\n\
                This is a {}, so ensure the message reflects the scope of changes.\n\
                For initial commits, focus on key architectural decisions and stay under 72 characters.\n\
                Validate your message against the examples and rules before returning it.\n\
                Only return the commit message, nothing else.\n\n\
                Diff:\n{}".to_string(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            repo_path: PathBuf::from("."),
            include_untracked: true,
            show_diff: false,
        }
    }
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            allowed_types: vec![
                "feat".to_string(),
                "fix".to_string(),
                "docs".to_string(),
                "style".to_string(),
                "refactor".to_string(),
                "perf".to_string(),
                "test".to_string(),
                "build".to_string(),
                "ci".to_string(),
                "chore".to_string(),
                "revert".to_string(),
            ],
            max_length: 72,
            auto_commit: false,
            verify_format: true,
            require_confirmation: true,
        }
    }
}

pub const AVAILABLE_MODELS: &[(&str, &str)] = &[
    ("mistralai/Mixtral-8x7B-Instruct-v0.1", "Best overall performance, recommended default"),
    ("meta-llama/Llama-2-70b-chat-hf", "Excellent for detailed analysis"),
    ("mistralai/Mistral-7B-Instruct-v0.2", "Fast and efficient"),
    ("NousResearch/Nous-Hermes-2-Mixtral-8x7B-DPO", "Optimized for coding tasks"),
    ("openchat/openchat-3.5-0106", "Good balance of performance and speed"),
]; 