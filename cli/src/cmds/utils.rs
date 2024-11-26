use indicatif::{ProgressBar, ProgressStyle};
use std::{
    process::{Command, Stdio},
    time::Duration,
};

use colored::*;

pub fn install_tool(tool_name: &str, brew_package: &str) -> Result<(), String> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["-", "\\", "|", "/"]),
    );
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_message(format!("🔧 Installing {}", tool_name.yellow()));

    let status = Command::new("brew")
        .arg("install")
        .arg(brew_package)
        .stdout(Stdio::null()) // Suppress standard output
        .stderr(Stdio::null()) // Suppress standard error
        .status();

    spinner.finish_and_clear();

    match status {
        Ok(status) if status.success() => {
            println!(
                "{} {}",
                "✅".green(),
                format!("{} installed successfully!", tool_name.bold().blue())
            );
            Ok(())
        }
        Ok(status) => Err(format!(
            "{} {}",
            "❌".red(),
            format!(
                "Failed to install {} with exit code: {}",
                tool_name,
                status.code().unwrap_or(-1)
            )
            .red()
        )),
        Err(err) => Err(format!(
            "{} {}",
            "❌".red(),
            format!("Error executing brew install {}: {}", brew_package, err).red()
        )),
    }
}
