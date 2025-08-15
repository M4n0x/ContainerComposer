use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "container-compose")]
#[command(about = "A Docker Compose-like tool for Apple's container framework")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Path to the container-compose.yml file
    #[arg(short, long, default_value = "container-compose.yml")]
    pub file: String,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start and run containers (like docker-compose up)
    Up {
        /// Run in detached mode
        #[arg(short, long)]
        detach: bool,

        /// Recreate containers
        #[arg(long)]
        force_recreate: bool,
    },

    /// Stop and remove containers (like docker-compose down)
    Down {
        /// Remove volumes as well
        #[arg(short, long)]
        volumes: bool,
    },

    /// Show container logs
    Logs {
        /// Service name to show logs for (optional)
        service: Option<String>,

        /// Follow log output
        #[arg(short, long)]
        follow: bool,

        /// Number of lines to show from the end
        #[arg(long)]
        tail: Option<usize>,
    },

    /// List containers
    Ps,

    /// Build or rebuild services
    Build {
        /// Service name to build (optional)
        service: Option<String>,

        /// Don't use cache when building
        #[arg(long)]
        no_cache: bool,
    },

    /// Execute a command in a running container
    Exec {
        /// Service name
        service: String,

        /// Command to execute
        command: Vec<String>,

        /// Keep STDIN open even if not attached
        #[arg(short, long)]
        interactive: bool,

        /// Allocate a pseudo-TTY
        #[arg(short, long)]
        tty: bool,
    },

    /// Pull images for services
    Pull {
        /// Service name to pull (optional)
        service: Option<String>,
    },

    /// Restart services
    Restart {
        /// Service name to restart (optional)
        service: Option<String>,
    },

    /// Stop services
    Stop {
        /// Service name to stop (optional)
        service: Option<String>,
    },

    /// Start services
    Start {
        /// Service name to start (optional)
        service: Option<String>,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
