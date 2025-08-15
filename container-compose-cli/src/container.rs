use crate::config::{ContainerComposeConfig, Service};
use crate::ui::UI;
use anyhow::Result;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command as AsyncCommand;
use tokio::time::{Duration, timeout};

// Enum in Rust - like constants but more powerful
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerStatus {
    Running,
}

// Struct to represent a running container
#[derive(Debug, Clone)]
pub struct Container {
    pub status: ContainerStatus,
    pub container_id: Option<String>,
}

// Main container manager
pub struct ContainerManager {
    containers: HashMap<String, Container>,
    config: ContainerComposeConfig,
}

impl ContainerManager {
    pub fn new(config: ContainerComposeConfig) -> Self {
        Self {
            containers: HashMap::new(),
            config,
        }
    }

    // Start all services (like docker-compose up)
    pub async fn up(&mut self, ui: &UI, verbose: bool) -> Result<()> {
        ui.info("Starting container-compose services");

        // Initialize named volumes first
        self.initialize_volumes().await?;
        if verbose && !self.config.volumes.is_empty() {
            ui.info(&format!(
                "Initialized {} named volume(s)",
                self.config.volumes.len()
            ));
        }

        // Get service start order based on dependencies
        let start_order = self.get_start_order()?;

        let mut started_count = 0;
        for service_name in start_order {
            self.start_service_with_progress(&service_name, ui, verbose)
                .await?;
            started_count += 1;
        }

        ui.success(&format!("Started {} service(s)", started_count));
        Ok(())
    }

    // Stop all services (like docker-compose down)
    pub async fn down(&mut self, ui: &UI, verbose: bool) -> Result<()> {
        ui.info("Stopping container-compose services");

        // Get all containers that exist (running and stopped) for our services
        let existing_containers = self.get_all_service_containers().await?;

        if existing_containers.is_empty() {
            ui.info("No containers to stop");
            return Ok(());
        }

        // Stop in reverse order - process all existing containers
        let mut service_names: Vec<String> = self.config.services.keys().cloned().collect();
        service_names.reverse();
        service_names.retain(|name| existing_containers.contains(name));

        for service_name in service_names {
            self.stop_service_with_progress(&service_name, ui, verbose)
                .await?;
        }

        ui.success(&format!(
            "Processed {} service(s)",
            existing_containers.len()
        ));
        Ok(())
    }

    // Start a specific service with progress bar
    async fn start_service_with_progress(
        &mut self,
        service_name: &str,
        ui: &UI,
        verbose: bool,
    ) -> Result<()> {
        // Clone the service to avoid borrowing issues
        let service = self
            .config
            .services
            .get(service_name)
            .ok_or_else(|| anyhow::anyhow!("Service '{}' not found", service_name))?
            .clone();

        // Check if service is already running
        if let Some(container) = self.containers.get(service_name) {
            if container.status == ContainerStatus::Running {
                ui.inline_warning(&format!("{} already running", service_name));
                return Ok(());
            }
        }

        // Create progress bar for starting
        let pb = ui.create_start_progress(service_name);

        if verbose {
            println!(); // New line for verbose output
            // Show port mapping warnings in verbose mode
            if !service.ports.is_empty() {
                ui.warning(
                    "Port mappings are not directly supported by Apple's container framework",
                );
                ui.warning(&format!("Ports specified: {:?}", service.ports));
                ui.warning("You may need to configure networking separately");
            }
        }

        // Use Apple's container command to start the service
        let container_id = self
            .run_container_with_progress(service_name, &service, ui, verbose)
            .await?;

        // Finish progress bar and show result
        pb.finish_and_clear();

        let container = Container {
            status: ContainerStatus::Running,
            container_id: Some(container_id.clone()),
        };

        self.containers.insert(service_name.to_string(), container);
        ui.inline_success(&format!("{} started ({})", service_name, container_id));

        Ok(())
    }

    // Get the order to start services based on dependencies
    fn get_start_order(&self) -> Result<Vec<String>> {
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();

        for service_name in self.config.services.keys() {
            if !visited.contains(service_name) {
                self.visit_service(service_name, &mut order, &mut visited, &mut visiting)?;
            }
        }

        Ok(order)
    }

    // Recursive function for topological sort (dependency resolution)
    fn visit_service(
        &self,
        service_name: &str,
        order: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        visiting: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        if visiting.contains(service_name) {
            return Err(anyhow::anyhow!(
                "Circular dependency detected involving '{}'",
                service_name
            ));
        }

        if visited.contains(service_name) {
            return Ok(());
        }

        visiting.insert(service_name.to_string());

        if let Some(service) = self.config.services.get(service_name) {
            for dep in &service.depends_on {
                self.visit_service(dep, order, visited, visiting)?;
            }
        }

        visiting.remove(service_name);
        visited.insert(service_name.to_string());
        order.push(service_name.to_string());

        Ok(())
    }

    // Get logs from a service
    pub async fn logs(&self, service_name: &str, follow: bool) -> Result<()> {
        if let Some(container) = self.containers.get(service_name) {
            if let Some(container_id) = &container.container_id {
                let mut cmd = AsyncCommand::new("container");
                cmd.args(&["logs"]);

                if follow {
                    cmd.arg("-f");
                }

                cmd.arg(container_id);
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());

                let status = cmd.status().await?;

                if !status.success() {
                    return Err(anyhow::anyhow!(
                        "Failed to get logs for service '{}'",
                        service_name
                    ));
                }
            } else {
                return Err(anyhow::anyhow!("Service '{}' is not running", service_name));
            }
        } else {
            return Err(anyhow::anyhow!("Service '{}' not found", service_name));
        }

        Ok(())
    }

    // Pull images for services
    pub async fn pull(&self, service_name: Option<String>, ui: &UI, verbose: bool) -> Result<()> {
        let services_to_pull = if let Some(name) = service_name {
            // Pull specific service
            if let Some(service) = self.config.services.get(&name) {
                vec![(name, service)]
            } else {
                return Err(anyhow::anyhow!("Service '{}' not found", name));
            }
        } else {
            // Pull all services
            self.config
                .services
                .iter()
                .map(|(k, v)| (k.clone(), v))
                .collect()
        };

        for (name, service) in services_to_pull {
            ui.info(&format!("Pulling image for service '{}'", name));
            self.pull_image(&service.image, ui, verbose).await?;
        }

        ui.success("All images pulled successfully");
        Ok(())
    }

    // Pull a specific image
    async fn pull_image(&self, image: &str, ui: &UI, verbose: bool) -> Result<()> {
        let mut cmd = AsyncCommand::new("container");
        cmd.args(&["images", "pull", image]);

        if verbose {
            ui.command(&format!("container images pull {}", image));
        }

        // Create progress bar
        let pb = ui.create_pull_progress(image);

        let output = cmd.output().await?;

        pb.finish_and_clear();

        if output.status.success() {
            ui.success(&format!("Successfully pulled: {}", image));
            // Print any output from the pull command if verbose
            if verbose && !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "Failed to pull image '{}': {}",
                image,
                error_msg
            ));
        }

        Ok(())
    }

    // Run a container with progress (used by start_service_with_progress)
    async fn run_container_with_progress(
        &self,
        name: &str,
        service: &Service,
        ui: &UI,
        verbose: bool,
    ) -> Result<String> {
        let mut cmd = AsyncCommand::new("container");
        cmd.args(&["run", "--detach", "--name", name]);

        // Add volume mounts (handle both bind mounts and named volumes)
        for volume in &service.volumes {
            let volume_spec = self.process_volume_mount(volume)?;
            cmd.args(&["--volume", &volume_spec]);
        }

        // Add environment variables
        for env in &service.environment {
            cmd.args(&["--env", env]);
        }

        // Set working directory if specified
        if let Some(working_dir) = &service.working_dir {
            cmd.args(&["--workdir", working_dir]);
        }

        // Add the image
        cmd.arg(&service.image);

        // Add command if specified
        if let Some(command) = &service.command {
            cmd.args(command);
        }

        if verbose {
            ui.command(&format!("{:?}", cmd));
        }

        let output = cmd.output().await?;

        if output.status.success() {
            let container_id = String::from_utf8(output.stdout)?.trim().to_string();
            Ok(container_id)
        } else {
            Err(anyhow::anyhow!(
                "Failed to start container '{}': {}",
                name,
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    // Get list of running containers
    async fn get_running_containers(&self) -> Result<Vec<String>> {
        let output = AsyncCommand::new("container")
            .args(&["list"])
            .output()
            .await?;

        if output.status.success() {
            let container_list = String::from_utf8_lossy(&output.stdout);
            let running_containers: Vec<String> = container_list
                .lines()
                .skip(1) // Skip header line
                .filter(|line| !line.is_empty())
                .filter_map(|line| {
                    // Parse the first column (ID/name) from container list
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 0 {
                        let container_name = parts[0];
                        // Only include if it's one of our services
                        if self.config.services.contains_key(container_name) {
                            Some(container_name.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            Ok(running_containers)
        } else {
            Ok(Vec::new())
        }
    }

    // Get list of all containers (running and stopped) for our services
    async fn get_all_service_containers(&self) -> Result<Vec<String>> {
        let output = AsyncCommand::new("container")
            .args(&["list", "--all"]) // Include stopped containers
            .output()
            .await?;

        if output.status.success() {
            let container_list = String::from_utf8_lossy(&output.stdout);
            let all_containers: Vec<String> = container_list
                .lines()
                .skip(1) // Skip header line
                .filter(|line| !line.is_empty())
                .filter_map(|line| {
                    // Parse the first column (ID/name) from container list
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 0 {
                        let container_name = parts[0];
                        // Only include if it's one of our services
                        if self.config.services.contains_key(container_name) {
                            Some(container_name.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            Ok(all_containers)
        } else {
            Ok(Vec::new())
        }
    }

    // Stop a service with progress bar
    async fn stop_service_with_progress(
        &mut self,
        service_name: &str,
        ui: &UI,
        verbose: bool,
    ) -> Result<()> {
        // Create progress bar for stopping
        let pb = ui.create_stop_progress(service_name);

        if verbose {
            println!(); // New line for verbose output
            ui.command(&format!("container stop {}", service_name));
        }

        // Try to stop the container gracefully first with timeout
        let stop_result = timeout(
            Duration::from_secs(10),
            AsyncCommand::new("container")
                .args(&["stop", service_name])
                .output(),
        )
        .await;

        let mut output = match stop_result {
            Ok(result) => result?,
            Err(_) => {
                // Timeout - container is not responding, force kill
                if verbose {
                    ui.command(&format!("container kill {} (timeout)", service_name));
                }
                AsyncCommand::new("container")
                    .args(&["kill", service_name])
                    .output()
                    .await?
            }
        };

        // If graceful stop failed, try force kill
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            if !error_msg.contains("no such container") && !error_msg.contains("not found") {
                if verbose {
                    ui.command(&format!("container kill {}", service_name));
                }

                // Try force kill
                output = AsyncCommand::new("container")
                    .args(&["kill", service_name])
                    .output()
                    .await?;

                // If kill also failed, try one more time after a brief delay
                if !output.status.success() && verbose {
                    ui.command(&format!("container kill {} (retry)", service_name));
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    output = AsyncCommand::new("container")
                        .args(&["kill", service_name])
                        .output()
                        .await?;
                }
            }
        }

        // Finish progress bar and show result
        pb.finish_and_clear();

        if output.status.success() {
            ui.inline_success(&format!("{} stopped", service_name));

            // Also try to remove the container
            if verbose {
                ui.command(&format!("container rm {}", service_name));
            }

            let _rm_output = AsyncCommand::new("container")
                .args(&["rm", service_name])
                .output()
                .await;
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            if error_msg.contains("no such container") || error_msg.contains("not found") {
                ui.inline_info(&format!("{} not found", service_name));
            } else {
                ui.inline_warning(&format!(
                    "{} failed to stop (tried stop and kill)",
                    service_name
                ));
            }
        }

        Ok(())
    }

    // List all services and their status
    pub async fn ps(&self, ui: &UI) -> Result<()> {
        // Get all containers (running and stopped) for our services
        let all_containers = self.get_all_service_containers().await?;
        let running_containers = self.get_running_containers().await?;

        ui.table_header(&["SERVICE", "STATUS", "CONTAINER ID", "IMAGE"]);

        // Process each service defined in the config
        for (service_name, service) in &self.config.services {
            if all_containers.contains(service_name) {
                // Container exists - determine if it's running or stopped
                let is_running = running_containers.contains(service_name);
                let status = if is_running { "Running" } else { "Stopped" };
                let status_color = if is_running {
                    Some("green")
                } else {
                    Some("red")
                };

                // Get container details
                let container_details = self.get_container_details(service_name).await?;
                ui.table_row(
                    &[
                        service_name,
                        status,
                        &container_details.0, // container ID
                        &container_details.1, // image
                    ],
                    status_color,
                );
            } else {
                // No container exists for this service
                ui.table_row(
                    &[service_name, "Not Created", "N/A", &service.image],
                    Some("red"),
                );
            }
        }

        Ok(())
    }

    // Get container details (ID and image) from Apple's container list
    async fn get_container_details(&self, service_name: &str) -> Result<(String, String)> {
        let output = AsyncCommand::new("container")
            .args(&["list", "--all"]) // Include stopped containers
            .output()
            .await?;

        if output.status.success() {
            let container_info = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = container_info.trim().lines().collect();

            // Skip header line and find our container
            for line in lines.iter().skip(1) {
                if !line.is_empty() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 && parts[0] == service_name {
                        // parts[0] = ID/Name, parts[1] = Image
                        let container_id = parts[0].to_string();
                        let image = parts[1].to_string();
                        return Ok((container_id, image));
                    }
                }
            }
        }

        // If we can't find it in containers, get image from config
        if let Some(service) = self.config.services.get(service_name) {
            Ok((service_name.to_string(), service.image.clone()))
        } else {
            Ok((service_name.to_string(), "unknown".to_string()))
        }
    }

    // Process volume mount - handle named volumes and bind mounts
    fn process_volume_mount(&self, volume: &str) -> Result<String> {
        if volume.contains(':') {
            let parts: Vec<&str> = volume.split(':').collect();
            if parts.len() >= 2 {
                let host_path = parts[0];
                let container_path = parts[1];
                let rest = if parts.len() > 2 {
                    format!(":{}", parts[2])
                } else {
                    String::new()
                };

                let abs_host_path = if self.is_named_volume(host_path) {
                    // Named volume - create managed directory
                    self.get_named_volume_path(host_path)?
                } else {
                    // Bind mount - convert relative path to absolute
                    let resolved_path = if host_path.starts_with("./")
                        || (!host_path.starts_with('/') && !host_path.contains('/'))
                    {
                        let current_dir = std::env::current_dir()?;
                        let path = if host_path.starts_with("./") {
                            current_dir.join(&host_path[2..])
                        } else {
                            current_dir.join(host_path)
                        };
                        path.to_string_lossy().to_string()
                    } else {
                        host_path.to_string()
                    };

                    // Validate that the source path exists
                    let path = std::path::Path::new(&resolved_path);
                    if !path.exists() {
                        return Err(anyhow::anyhow!(
                            "Volume mount source path does not exist: {} (resolved to: {})",
                            host_path,
                            resolved_path
                        ));
                    }

                    // Warn if path contains spaces (potential issue with Apple's container framework)
                    if resolved_path.contains(' ') {
                        eprintln!(
                            "Warning: Volume path contains spaces, this may cause issues: {}",
                            resolved_path
                        );
                    }

                    resolved_path
                };

                Ok(format!("{}:{}{}", abs_host_path, container_path, rest))
            } else {
                Ok(volume.to_string())
            }
        } else {
            // Anonymous volume - not supported by Apple's container framework
            Err(anyhow::anyhow!(
                "Anonymous volumes are not supported: {}",
                volume
            ))
        }
    }

    // Check if a volume name is a named volume (defined in config.volumes)
    fn is_named_volume(&self, volume_name: &str) -> bool {
        self.config.volumes.contains_key(volume_name)
    }

    // Get the host path for a named volume
    fn get_named_volume_path(&self, volume_name: &str) -> Result<String> {
        // Use a global volumes directory in user's home directory for consistency
        let home_dir = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;

        let volumes_dir = std::path::Path::new(&home_dir)
            .join(".container-compose")
            .join("volumes")
            .join(volume_name);

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(&volumes_dir)?;

        Ok(volumes_dir.to_string_lossy().to_string())
    }

    // Initialize named volumes (create directories)
    pub async fn initialize_volumes(&self) -> Result<()> {
        for volume_name in self.config.volumes.keys() {
            let _volume_path = self.get_named_volume_path(volume_name)?;
            // Volume directory is created in get_named_volume_path
        }
        Ok(())
    }

    // Execute a command in a running container
    pub async fn exec(
        &self,
        service_name: &str,
        command: &[String],
        ui: &UI,
        verbose: bool,
    ) -> Result<()> {
        // Check if service exists in config
        if !self.config.services.contains_key(service_name) {
            return Err(anyhow::anyhow!("Service '{}' not found", service_name));
        }

        if verbose {
            ui.command(&format!(
                "container exec {} {}",
                service_name,
                command.join(" ")
            ));
        }

        // Execute command using Apple's container framework
        let mut cmd = AsyncCommand::new("container");
        cmd.args(&["exec", service_name]);
        cmd.args(command);

        // Inherit stdin, stdout, stderr for interactive usage
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let status = cmd.status().await?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Command failed in container '{}' with exit code: {}",
                service_name,
                status.code().unwrap_or(-1)
            ));
        }

        Ok(())
    }
}
