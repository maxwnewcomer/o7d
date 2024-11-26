use std::fs;
use std::path::Path;

pub fn init_dir(custom_envs: Vec<&str>) {
    // Use custom environments if provided, otherwise use defaults
    let environments = if custom_envs.is_empty() {
        vec!["prod", "dev"]
    } else {
        custom_envs
    };

    let base_dirs = vec!["apps", "infra", "clusters"];

    // Create the base directory structure
    for dir in base_dirs {
        // Create the main directory
        let base_path = Path::new(dir);
        if let Err(e) = fs::create_dir_all(base_path) {
            eprintln!("Error creating directory {}: {}", dir, e);
            continue;
        }

        // Create 'base' subdirectory
        let base_subdir = base_path.join("base");
        if let Err(e) = fs::create_dir_all(&base_subdir) {
            eprintln!("Error creating base subdirectory in {}: {}", dir, e);
        }

        // Create environment subdirectories
        for env in &environments {
            let env_subdir = base_path.join(env);
            if let Err(e) = fs::create_dir_all(&env_subdir) {
                eprintln!("Error creating {} subdirectory in {}: {}", env, dir, e);
            }
        }
    }

    println!("Created environments: base, {}", environments.join(", "));
    println!("✨ Repository structure created successfully!");
}
