use crate::cmds::utils;

pub fn run_setup() {
    println!("Setting up Kubernetes tools...");

    if let Err(err) = utils::install_tool("KIND", "kind") {
        eprintln!("{}", err);
    }

    if let Err(err) = utils::install_tool("kubectl", "kubectl") {
        eprintln!("{}", err);
    }

    println!("✨ Setup complete!");
}
