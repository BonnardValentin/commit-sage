pub mod ai;
pub mod config;
pub mod error;
pub mod git;
pub mod protocol;

pub use crate::ai::AiClient;
pub use crate::config::{Config, AiConfig, GitConfig, CommitConfig, AVAILABLE_MODELS};
pub use crate::error::{Error, Result};
pub use crate::git::GitRepo;
pub use crate::protocol::{
    ModelProvider, CommitMessageGenerator, ModelContext, GenerationConfig,
    Message, TogetherAiProvider,
};

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("feat: add new feature", true)]
    #[test_case("fix(core): resolve issue", true)]
    #[test_case("random message", false)]
    fn test_is_conventional_commit(message: &str, expected: bool) {
        let is_conventional = is_conventional_commit(message);
        assert_eq!(is_conventional, expected);
    }
}

/// Checks if a commit message follows the Conventional Commits specification
pub fn is_conventional_commit(message: &str) -> bool {
    let conventional_types = [
        "feat", "fix", "docs", "style", "refactor",
        "perf", "test", "build", "ci", "chore", "revert"
    ];

    // Basic format: <type>[optional scope]: <description>
    let parts: Vec<&str> = message.splitn(2, ": ").collect();
    if parts.len() != 2 {
        return false;
    }

    let type_part = parts[0];
    
    // Check if there's a scope
    let commit_type = if type_part.contains('(') {
        type_part.split('(').next().unwrap_or("")
    } else {
        type_part
    };

    conventional_types.contains(&commit_type)
} 