use indicatif::ProgressBar;
use std::{
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use rand::{thread_rng, Rng};

pub fn install_tool(pb: ProgressBar, tool_name: &str, brew_package: &str) {
    pb.set_message(format!("Installing {}", tool_name));
    pb.enable_steady_tick(Duration::from_millis(50));

    // Simulating installation time or calling a real command
    let status = Command::new("brew")
        .arg("install")
        .arg(brew_package)
        .stdout(Stdio::null()) // Suppress output
        .stderr(Stdio::null()) // Suppress output
        .status();

    let mut rng = thread_rng();

    thread::sleep(rng.gen_range(Duration::from_secs(1)..Duration::from_secs(5)));

    pb.finish_with_message(if let Ok(status) = status {
        if status.success() {
            format!("✅ {} installed successfully", tool_name)
        } else {
            format!("❌ Failed to install {}", tool_name)
        }
    } else {
        format!("❌ Error installing {}", tool_name)
    });
}
