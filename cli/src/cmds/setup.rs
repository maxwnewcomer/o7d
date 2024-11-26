use std::thread;

use crate::cmds::utils;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub fn run_setup() {
    // Define tools to install
    let tools = vec![
        ("KIND", "kind"),
        ("kubectl", "kubectl"),
        ("Helm", "helm"),
        ("Minikube", "minikube"),
    ];

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let mut threads = vec![];

    for (tool_name, brew_package) in tools {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(sty.clone());

        let tool_name = tool_name.to_string();
        let brew_package = brew_package.to_string();

        // Spawn a thread for each tool installation
        threads.push(thread::spawn(move || {
            utils::install_tool(pb, &tool_name, &brew_package);
        }));
    }

    // Wait for all threads to complete
    for thread in threads {
        let _ = thread.join();
    }

    // Clear all progress bars
    m.clear().unwrap();
    println!("✨ All installations are complete!");
}
