//! ZenTerm CLI Application
//! 
//! Command-line interface for ZenTerm with voice and theming capabilities.

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod commands;

use commands::*;

#[derive(Parser)]
#[command(name = "zenterm")]
#[command(about = "ZenTerm v2 - Terminal orchestrator with voice and theming")]
#[command(version = "0.1.0")]
struct Cli {
    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Voice input commands
    Voice {
        #[command(subcommand)]
        action: VoiceCommands,
    },
    /// Theme management commands
    Theme {
        #[command(subcommand)]
        action: ThemeCommands,
    },
    /// Elevation management commands
    Elevate {
        #[command(subcommand)]
        action: ElevateCommands,
    },
    /// Plugin management commands
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },
    /// Configuration commands
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum VoiceCommands {
    /// Start voice input mode
    Start {
        /// Use push-to-talk mode
        #[arg(long)]
        push_to_talk: bool,
    },
    /// Test microphone input
    TestMic,
}

#[derive(Subcommand)]
enum ThemeCommands {
    /// Generate theme from image or prompt
    Generate {
        /// Source image path or text prompt
        source: String,
        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },
    /// List available themes
    List,
}

#[derive(Subcommand)]
enum ElevateCommands {
    /// Check elevation status
    Status,
    /// Request elevated privileges
    Request {
        /// Duration in minutes
        #[arg(short, long, default_value = "5")]
        duration: u32,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// List available plugins
    List,
    /// Show plugin capabilities
    Capabilities {
        /// Plugin name
        name: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show configuration file path
    Path,
    /// Validate configuration
    Validate,
    /// Initialize default configuration
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize structured logging
    init_logging(&cli.log_level)?;
    
    info!("ZenTerm v2 starting");

    // Execute commands
    match cli.command {
        Commands::Voice { action } => handle_voice_command(action).await,
        Commands::Theme { action } => handle_theme_command(action).await,
        Commands::Elevate { action } => handle_elevate_command(action).await,
        Commands::Plugin { action } => handle_plugin_command(action).await,
        Commands::Config { action } => handle_config_command(action).await,
    }
}

fn init_logging(log_level: &str) -> Result<()> {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(level.into())
        )
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}