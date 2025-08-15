use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct UI;

impl UI {
    pub fn new() -> Self {
        Self
    }

    // Print a styled header
    pub fn header(&self, text: &str) {
        println!("{}", text.bright_blue().bold());
    }

    // Print a success message
    pub fn success(&self, text: &str) {
        println!("{} {}", "[✓]".green().bold(), text.green());
    }

    // Print an info message
    pub fn info(&self, text: &str) {
        println!("{} {}", "[i]".blue().bold(), text);
    }

    // Print a warning message
    pub fn warning(&self, text: &str) {
        println!("{} {}", "[!]".yellow().bold(), text.yellow());
    }

    // Print an error message
    pub fn error(&self, text: &str) {
        println!("{} {}", "[✗]".red().bold(), text.red().bold());
    }

    // Create a progress bar for image pulling
    pub fn create_pull_progress(&self, image: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} Pulling {msg}...")
                .unwrap(),
        );
        pb.set_message(image.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    // Create a progress bar for stopping containers
    pub fn create_stop_progress(&self, service: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.yellow} Stopping {msg}...")
                .unwrap(),
        );
        pb.set_message(service.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    // Create a progress bar for starting containers
    pub fn create_start_progress(&self, service: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.green} Starting {msg}...")
                .unwrap(),
        );
        pb.set_message(service.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    // Print a table header
    pub fn table_header(&self, headers: &[&str]) {
        let header_line = headers
            .iter()
            .map(|h| format!("{:<15}", h.bold()))
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", header_line);
        println!("{}", "-".repeat(header_line.len()).dimmed());
    }

    // Print a table row
    pub fn table_row(&self, cells: &[&str], status_color: Option<&str>) {
        let row = cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                if i == 1 && status_color.is_some() {
                    // Status column
                    match status_color.unwrap() {
                        "green" => format!("{:<15}", cell.green()),
                        "red" => format!("{:<15}", cell.red()),
                        "yellow" => format!("{:<15}", cell.yellow()),
                        _ => format!("{:<15}", cell),
                    }
                } else {
                    format!("{:<15}", cell)
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", row);
    }

    // Print command being executed (for verbose mode)
    pub fn command(&self, cmd: &str) {
        println!("{} {}", "[>]".cyan().bold(), cmd.dimmed());
    }

    // Print a separator line
    pub fn separator(&self) {
        println!("{}", "=".repeat(60).dimmed());
    }

    // Print inline success message
    pub fn inline_success(&self, text: &str) {
        println!("{} {}", "[✓]".green().bold(), text.green());
    }

    // Print inline info message
    pub fn inline_info(&self, text: &str) {
        println!("{} {}", "[i]".blue().bold(), text);
    }

    // Print inline warning message
    pub fn inline_warning(&self, text: &str) {
        println!("{} {}", "[!]".yellow().bold(), text.yellow());
    }
}
