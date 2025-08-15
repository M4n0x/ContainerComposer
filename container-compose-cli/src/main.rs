#![allow(clippy::uninlined_format_args)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::len_zero)]
#![allow(clippy::manual_strip)]

mod cli;
mod config;
mod container;
mod ui;

use anyhow::Result;
use cli::{Cli, Commands};
use config::ContainerComposeConfig;
use container::ContainerManager;
use ui::UI;
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::parse_args();

    // Create UI instance
    let ui = UI::new();

    // Print header
    ui.header("Container Compose v0.1.0");
    ui.info(&format!("Using config file: {}", args.file));

    // Load and validate configuration
    let config = match ContainerComposeConfig::from_file(&args.file) {
        Ok(config) => {
            ui.success("Configuration loaded successfully");
            config
        }
        Err(e) => {
            ui.error(&format!("Failed to load configuration: {e}"));
            std::process::exit(1);
        }
    };

    if let Err(e) = config.validate() {
        ui.error(&format!("Configuration validation failed: {e}"));
        std::process::exit(1);
    }

    // Create container manager
    let mut manager = ContainerManager::new(config);

    // Handle different commands
    let result = match args.command {
        Commands::Up {
            detach,
            force_recreate,
        } => {
            ui.separator();
            ui.info(&format!(
                "Starting services (detach: {detach}, force_recreate: {force_recreate})"
            ));
            manager.up(&ui, args.verbose).await
        }

        Commands::Down { volumes } => {
            ui.separator();
            ui.info(&format!("Stopping services (remove volumes: {volumes})"));
            manager.down(&ui, args.verbose).await
        }

        Commands::Logs {
            service,
            follow,
            tail: _,
        } => {
            match service {
                Some(service_name) => {
                    ui.info(&format!("Showing logs for service: {service_name}"));
                    manager.logs(&service_name, follow).await
                }
                None => {
                    ui.info("Showing logs for all services");
                    // TODO: Implement logs for all services
                    Ok(())
                }
            }
        }

        Commands::Ps => {
            ui.separator();
            manager.ps(&ui).await
        }

        Commands::Build { service, no_cache } => {
            ui.info(&format!(
                "Building services (service: {service:?}, no_cache: {no_cache})"
            ));
            // TODO: Implement build functionality
            Ok(())
        }

        Commands::Exec {
            service,
            command,
            interactive: _,
            tty: _,
        } => {
            if command.is_empty() {
                // Default to shell if no command provided
                let default_command = vec!["sh".to_string()];
                manager
                    .exec(&service, &default_command, &ui, args.verbose)
                    .await
            } else {
                manager.exec(&service, &command, &ui, args.verbose).await
            }
        }

        Commands::Pull { service } => {
            ui.separator();
            ui.info(&format!("Pulling images (service: {service:?})"));
            manager.pull(service, &ui, args.verbose).await
        }

        Commands::Restart { service } => {
            ui.info(&format!("Restarting services (service: {service:?})"));
            // TODO: Implement restart functionality
            Ok(())
        }

        Commands::Stop { service } => {
            ui.info(&format!("Stopping services (service: {service:?})"));
            // TODO: Implement stop functionality
            Ok(())
        }

        Commands::Start { service } => {
            ui.info(&format!("Starting services (service: {service:?})"));
            // TODO: Implement start functionality
            Ok(())
        }
    };

    if let Err(e) = result {
        ui.error(&format!("Command failed: {e}"));
        std::process::exit(1);
    }

    Ok(())
}
