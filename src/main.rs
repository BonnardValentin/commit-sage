use std::path::PathBuf;
use clap::Parser;
use git_commit_sage::{
    AiClient, GitRepo, Config, Error, Result, AVAILABLE_MODELS,
    is_conventional_commit,
};
use tracing::{info, warn};
use std::io::{self, Write};

/// A smart Git commit message generator using AI
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the git repository (defaults to current directory)
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Together.ai API key
    #[arg(short = 'k', long, env = "TOGETHER_API_KEY")]
    api_key: Option<String>,

    /// AI model to use
    #[arg(short, long)]
    model: Option<String>,

    /// Temperature for model output (0.0 to 1.0)
    #[arg(short = 't', long, default_value = "0.3")]
    temperature: f32,

    /// Maximum tokens in response
    #[arg(long, default_value = "100")]
    max_tokens: u32,

    /// Include untracked files in diff
    #[arg(short, long)]
    untracked: bool,

    /// Show diff before generating commit message
    #[arg(short, long)]
    show_diff: bool,

    /// Automatically commit with generated message
    #[arg(short = 'a', long)]
    auto_commit: bool,

    /// Skip commit message format verification
    #[arg(long)]
    no_verify: bool,

    /// Skip user confirmation
    #[arg(short = 'y', long)]
    yes: bool,

    /// Path to custom configuration file
    #[arg(short = 'f', long)]
    config: Option<PathBuf>,

    /// List available models
    #[arg(short, long)]
    list_models: bool,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Parse command line arguments
    let args = Args::parse();

    // List available models if requested
    if args.list_models {
        println!("Available models:");
        for (model, description) in AVAILABLE_MODELS {
            println!("  {} - {}", model, description);
        }
        return Ok(());
    }

    // Setup logging
    setup_logging(args.debug);

    // Load configuration
    let mut config = if let Some(config_path) = args.config {
        info!("Loading configuration from {}", config_path.display());
        let config_str = std::fs::read_to_string(config_path)?;
        toml::from_str(&config_str)?
    } else {
        Config::default()
    };

    // Override configuration with command line arguments
    if let Some(path) = args.path {
        config.git.repo_path = path;
    }
    if let Some(model) = args.model {
        config.ai.model = model;
    }
    config.ai.temperature = args.temperature;
    config.ai.max_tokens = args.max_tokens;
    config.git.include_untracked = args.untracked;
    config.git.show_diff = args.show_diff;
    config.commit.auto_commit = args.auto_commit;
    config.commit.verify_format = !args.no_verify;
    config.commit.require_confirmation = !args.yes;

    info!("Opening git repository at {}", config.git.repo_path.display());
    
    // Initialize git repository
    let repo = GitRepo::new(config.git.clone())?;

    // Check for changes
    if !repo.has_changes()? {
        warn!("No changes to commit!");
        return Err(Error::NoChanges);
    }

    // Get API key
    let api_key = args.api_key
        .or_else(|| std::env::var("TOGETHER_API_KEY").ok())
        .ok_or_else(|| Error::NoApiKey)?;

    // Initialize AI client
    let ai_client = AiClient::new(api_key, config.ai.clone());

    // Get diff
    info!("Getting git diff");
    let diff = repo.get_diff()?;

    // Show diff if requested
    if config.git.show_diff {
        println!("\nChanges to be committed:\n{}", diff);
    }

    // Generate commit message
    info!("Generating commit message using model {}", config.ai.model);
    let commit_message = ai_client.generate_commit_message(&diff).await?;

    // Verify commit message format if enabled
    if config.commit.verify_format && !is_conventional_commit(&commit_message) {
        return Err(Error::CommitMessageGeneration(
            "Generated message does not follow conventional commit format".to_string(),
        ));
    }

    // Print result
    println!("\nSuggested commit message:\n{}", commit_message);

    // Auto-commit if enabled and confirmation is received
    if config.commit.auto_commit {
        if config.commit.require_confirmation {
            print!("\nDo you want to commit with this message? [y/N] ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                println!("Commit aborted.");
                return Ok(());
            }
        }
        
        info!("Auto-committing changes");
        repo.commit(&commit_message)?;
        println!("Changes committed successfully!");
    }

    Ok(())
}

fn setup_logging(debug: bool) {
    let filter = if debug { "debug" } else { "info" };
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}