# 🧙‍♂️ Git Commit Sage

> 🤖 Your AI-powered companion for writing perfect conventional commit messages

[![Crates.io](https://img.shields.io/crates/v/git-commit-sage.svg)](https://crates.io/crates/git-commit-sage)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🎯 Generates conventional commit messages from your git diff
- 🔄 Supports multiple AI providers through a flexible trait system
- ⚡ Built-in support for Together.ai's Mixtral-8x7B model
- 🛠️ Configurable via TOML and environment variables
- 📦 Available as both a CLI tool and a Rust library

## 🚀 Installation

### As a CLI Tool

```bash
# Install via cargo
$ cargo install git-commit-sage
    Updating crates.io index
  Downloaded git-commit-sage v0.1.0
   Compiling git-commit-sage v0.1.0
    Finished release [optimized] target(s) in 1m 12s
  Installing ~/.cargo/bin/git-commit-sage
   Installed package `git-commit-sage v0.1.0` 

# Or build from source
$ git clone https://github.com/yourusername/git-commit-sage
Cloning into 'git-commit-sage'...
done.

$ cd git-commit-sage
$ cargo install --path .
    Finished release [optimized] target(s) in 1m 08s
  Installing git-commit-sage
   Installed package `git-commit-sage v0.1.0` (executable `git-commit-sage`)
```

### Updating

```bash
# If installed from crates.io
$ cargo install git-commit-sage --force
    Updating crates.io index
  Downloaded git-commit-sage v0.1.1
   Compiling git-commit-sage v0.1.1
    Finished release [optimized]
  Replacing ~/.cargo/bin/git-commit-sage
   Installed package `git-commit-sage v0.1.1`

# If installed from source
$ cd git-commit-sage
$ git pull  # Get latest changes
$ cargo install --path . --force
    Finished release [optimized]
  Replacing ~/.cargo/bin/git-commit-sage
   Installed package `git-commit-sage v0.1.1`

# Verify the installation
$ git-commit-sage --version
git-commit-sage 0.1.1
```

### Uninstallation

```bash
$ cargo uninstall git-commit-sage
    Removing ~/.cargo/bin/git-commit-sage
```

## 🎯 Getting Started

### First Time Setup

```bash
# 1. Initialize your repository (if not already done)
$ git init
Initialized empty Git repository in .../your-project/.git/

# 2. Create your .env file with your API key
$ echo "TOGETHER_API_KEY=your_api_key_here" > .env

# 3. Stage ALL your files
$ git add .
$ git status  # Verify all files are staged
Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
        new file:   .gitignore
        new file:   Cargo.toml
        new file:   README.md
        ...

# 4. Create the initial commit manually (required for first commit only)
$ git commit -m "chore: initial commit"
[main (root-commit)] chore: initial commit
 11 files changed, 523 insertions(+)
 ...

# 5. For subsequent changes, stage and use git-commit-sage
$ echo "# New section" >> README.md
$ git add .  # Stage ALL changes
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: docs(readme): add new section header
```

### Quick Start Guide

1. **Stage your changes**
   ```bash
   # Always stage ALL related changes before generating a message
   $ git add .  # Stage all changes in the repository
   # OR
   $ git add src/feature/* test/feature/*  # Stage specific related files
   
   # Verify what's staged
   $ git status
   Changes to be committed:
     modified:   src/feature/main.rs
     modified:   test/feature/test.rs
   ```

2. **Generate a commit message**
   ```bash
   $ git-commit-sage
   ✨ Analyzing git diff...
   🤖 Generating commit message...
   📝 Suggested commit message: feat(core): implement user authentication with tests
   ```

3. **Review and commit**
   ```bash
   # Option 1: Manual commit with the suggested message
   $ git commit -m "feat(core): implement user authentication with tests"
   
   # Option 2: Auto-commit (uses the staged changes)
   $ git-commit-sage -a
   ```

### Best Practices

1. **Stage Related Changes Together**
   - Always stage ALL related files before generating a message
   - Use `git status` to verify what's staged
   - The better your staging, the better the commit message

2. **Review the Diff**
   ```bash
   # See what changes will be included in the message generation
   $ git-commit-sage --show-diff
   
   # Or use git's built-in diff tool
   $ git diff --cached  # Show all staged changes
   ```

3. **Adjust Message Quality**
   ```bash
   # Use lower temperature for more focused messages
   $ git-commit-sage -t 0.2
   
   # Use higher temperature for more creative messages
   $ git-commit-sage -t 0.8
   ```

4. **Working with Multiple Changes**
   ```bash
   # Bad: Staging unrelated changes
   $ git add src/auth.rs src/logging.rs  # Unrelated changes
   
   # Good: Stage related changes together
   $ git add src/auth.rs src/auth_test.rs  # Related auth changes
   $ git-commit-sage  # Generate message for auth changes
   
   $ git add src/logging.rs  # Stage logging changes separately
   $ git-commit-sage  # Generate separate message for logging
   ```

## 🔧 Configuration

1. Create a `.env` file or set your environment variables:
```bash
$ echo "TOGETHER_API_KEY=your_api_key_here" > .env
$ cat .env
TOGETHER_API_KEY=your_api_key_here
```

2. (Optional) Create a `commit-sage.toml` in your project root or home directory:
```bash
$ cat > commit-sage.toml << EOF
[ai]
provider = "together"
model = "mistralai/Mixtral-8x7B-Instruct-v0.1"
temperature = 0.3

[commit]
auto_commit = false
validate = true
EOF
```

## 💻 Usage

### CLI Usage

```bash
# Generate a commit message for staged changes
$ git add .
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: feat(auth): implement OAuth2 authentication flow

# Show the diff being analyzed (useful for debugging)
$ git-commit-sage --show-diff
✨ Analyzing git diff...
diff --git a/src/auth.rs b/src/auth.rs
...

# Generate and automatically commit
$ git-commit-sage -a
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Generated: feat(api): add rate limiting middleware
✅ Changes committed successfully!

# Specify custom temperature
$ git-commit-sage -t 0.5
✨ Analyzing git diff...
🤖 Generating commit message (temperature: 0.5)...
📝 Suggested commit message: refactor(core): optimize database queries

# Use a different API key
$ git-commit-sage -k your_api_key
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: fix(ui): resolve responsive layout issues
```

### Library Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
git-commit-sage = "0.1.0"
```

Example implementation:

```rust
use git_commit_sage::{
    TogetherAiProvider, CommitMessageGenerator,
    ModelProvider, GenerationConfig
};
use async_trait::async_trait;

// Use the built-in Together.ai provider
let provider = TogetherAiProvider::new(
    "your_api_key".to_string(),
    "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string()
);

// Or implement your own provider
struct CustomProvider;

#[async_trait]
impl ModelProvider for CustomProvider {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn generate(&self, context: ModelContext) -> Result<String, Self::Error> {
        // Your implementation here
    }

    fn model_id(&self) -> &str {
        "custom-model"
    }

    fn default_config(&self) -> GenerationConfig {
        GenerationConfig {
            temperature: 0.3,
            max_tokens: 100,
            stop_sequences: vec!["\n".to_string()],
        }
    }
}
```

## 🔄 Common Workflows

### Feature Development

```bash
# Start a new feature branch
$ git checkout -b feature/user-authentication
Switched to a new branch 'feature/user-authentication'

# Make your changes and stage them
$ git add src/auth.rs src/models/user.rs
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: feat(auth): implement user authentication middleware

# Make more changes
$ git add src/config/auth.rs
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: feat(config): add JWT configuration options

# Final changes with auto-commit
$ git add .
$ git-commit-sage -a
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Generated: feat(auth): add password reset functionality
✅ Changes committed successfully!
```

### Bug Fixing

```bash
# Create a bug fix branch
$ git checkout -b fix/api-timeout
Switched to a new branch 'fix/api-timeout'

# Fix the bug and stage changes
$ git add src/api/client.rs
$ git-commit-sage -t 0.2  # Lower temperature for more focused message
✨ Analyzing git diff...
🤖 Generating commit message (temperature: 0.2)...
📝 Suggested commit message: fix(api): increase timeout for long-running requests
```

### Refactoring

```bash
# Start refactoring
$ git checkout -b refactor/database-layer
Switched to a new branch 'refactor/database-layer'

# Stage partial changes
$ git add src/db/connection.rs
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: refactor(db): implement connection pooling

# Stage more changes
$ git add src/db/
$ git-commit-sage --show-diff  # Review changes before committing
✨ Analyzing git diff...
diff --git a/src/db/models.rs b/src/db/models.rs
...
🤖 Generating commit message...
📝 Suggested commit message: refactor(db): migrate to async database operations
```

### Documentation Updates

```bash
# Update docs
$ git checkout -b docs/api-reference
$ git add docs/
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: docs(api): update REST API documentation with new endpoints

# Update examples
$ git add examples/
$ git-commit-sage -a
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Generated: docs(examples): add authentication code samples
✅ Changes committed successfully!
```

### Project Maintenance

```bash
# Update dependencies
$ cargo update
$ git add Cargo.lock
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: chore(deps): update dependencies to latest versions

# Configure CI/CD
$ git add .github/workflows/
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: ci: add GitHub Actions workflow for automated testing
```

### Working with Multiple Changes

```bash
# Stage and commit related changes together
$ git add src/auth/
$ git add tests/auth/
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: feat(auth): implement OAuth provider with tests

# Stage and commit unrelated changes separately
$ git add src/logging/
$ git-commit-sage
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Suggested commit message: feat(logging): add structured logging with tracing

$ git add src/metrics/
$ git-commit-sage -a
✨ Analyzing git diff...
🤖 Generating commit message...
📝 Generated: feat(metrics): implement Prometheus metrics collection
✅ Changes committed successfully!
```

## 🚨 Troubleshooting

### Common Issues

1. **Error: NoChanges**
   ```bash
   $ git-commit-sage
   Error: NoChanges
   ```
   This can happen in two cases:
   - No changes are staged (run `git add` first)
   - This is the initial commit (you need at least one commit for diff comparison)
   
   For initial commits, you should create the first commit manually:
   ```bash
   $ git add .
   $ git commit -m "chore: initial commit"
   ```

2. **API Key Issues**
   ```bash
   $ git-commit-sage
   Error: Invalid API key
   ```
   Make sure your Together.ai API key is:
   - Correctly set in `.env` or environment variables
   - Valid and has sufficient credits
   - Not expired or revoked

3. **No Configuration File**
   ```bash
   $ git-commit-sage
   Warning: No config file found, using defaults
   ```
   This is normal! The tool works with sensible defaults, but you can create a config file:
   ```bash
   $ cp config.example.toml commit-sage.toml
   $ nano commit-sage.toml  # Edit configuration as needed
   ```

## 🌟 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Together.ai](https://together.ai)
- Uses the Mixtral-8x7B model by Mistral AI
- Inspired by the Conventional Commits specification 