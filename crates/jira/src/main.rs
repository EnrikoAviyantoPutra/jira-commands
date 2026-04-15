use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use jira_core::{auth::Auth, config::JiraConfig, JiraClient};
use tracing_subscriber::{fmt, EnvFilter};

mod cli;
mod tui;

#[derive(Debug, Parser)]
#[command(
    name = "jira",
    about = "Jira CLI — terminal client for Atlassian Jira",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Issue management
    Issue {
        #[command(subcommand)]
        command: cli::issue::IssueCommand,
    },
    /// Authentication management
    Auth {
        #[command(subcommand)]
        command: cli::auth::AuthCommand,
    },
    /// Launch interactive TUI
    Tui {
        /// Default project key
        #[arg(short, long)]
        project: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"))
    };

    fmt().with_env_filter(filter).with_target(false).init();

    match cli.command {
        Commands::Auth { command } => {
            cli::auth::handle(command).await?;
        }
        Commands::Issue { command } => {
            let client = build_client().context("Failed to initialize Jira client")?;
            let config = JiraConfig::load().unwrap_or_default();
            cli::issue::handle(command, client, config.project).await?;
        }
        Commands::Tui { project } => {
            let client = build_client().context("Failed to initialize Jira client")?;
            let config = JiraConfig::load().unwrap_or_default();
            let effective_project = project.or(config.project);
            tui::run_tui(client, effective_project)
                .await
                .context("TUI error")?;
        }
    }

    Ok(())
}

fn build_client() -> Result<JiraClient> {
    let mut config = JiraConfig::load().unwrap_or_default();

    if config.base_url.is_empty() {
        anyhow::bail!(
            "Jira URL not configured. Run `jira auth login` or set JIRA_URL environment variable."
        );
    }

    if config.email.is_empty() {
        anyhow::bail!(
            "Email not configured. Run `jira auth login` or set JIRA_EMAIL environment variable."
        );
    }

    // Load token from keyring if not in config/env
    if config.token.is_none() {
        match Auth::get_token(&config.email) {
            Ok(token) => config.token = Some(token),
            Err(_) => {
                anyhow::bail!(
                    "API token not found. Run `jira auth login` or set JIRA_TOKEN environment variable."
                );
            }
        }
    }

    Ok(JiraClient::new(config))
}
