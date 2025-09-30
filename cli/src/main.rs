use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "egui-desktop")]
#[command(about = "Initialize a new egui-desktop project")]
#[command(version)]
struct Cli {
    /// Project name
    name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("🚀 Creating new egui-desktop project: {}", cli.name);

    // Create project directory
    fs::create_dir_all(&cli.name)
        .with_context(|| format!("Failed to create directory: {}", cli.name))?;

    // Create src directory
    fs::create_dir_all(Path::new(&cli.name).join("src"))
        .with_context(|| "Failed to create src directory")?;

    // Copy all starter files
    let files_to_copy = vec![
        ("starter/main.rs", "main.rs"),
        ("starter/app.rs", "app.rs"),
        ("starter/theme_provider.rs", "theme_provider.rs"),
        ("starter/sidebar.rs", "sidebar.rs"),
        ("starter/content.rs", "content.rs"),
        ("starter/icons.rs", "icons.rs"),
        ("starter/lib.rs", "lib.rs"),
    ];

    for (source_file, target_file) in files_to_copy {
        let content = match source_file {
            "starter/main.rs" => include_str!("starter/main.rs"),
            "starter/app.rs" => include_str!("starter/app.rs"),
            "starter/theme_provider.rs" => include_str!("starter/theme_provider.rs"),
            "starter/sidebar.rs" => include_str!("starter/sidebar.rs"),
            "starter/content.rs" => include_str!("starter/content.rs"),
            "starter/icons.rs" => include_str!("starter/icons.rs"),
            "starter/lib.rs" => include_str!("starter/lib.rs"),
            _ => panic!("Unknown source file: {}", source_file),
        };

        // Replace template variables
        let content = content.replace("PROJECT_NAME_PLACEHOLDER", &cli.name.replace("-", "_"));

        let target_path = Path::new(&cli.name).join("src").join(target_file);
        fs::write(&target_path, content)
            .with_context(|| format!("Failed to write {}", target_file))?;
    }

    // Create Cargo.toml
    let cargo_toml_content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
egui-desktop = {{ path = "../../" }}
egui_extras = {{ version = "0.32", features = ["all_loaders"] }}
eframe = "0.32"
egui = "0.32"
"#,
        cli.name
    );

    let cargo_toml_path = Path::new(&cli.name).join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)
        .with_context(|| "Failed to write Cargo.toml")?;

    println!("✅ Project created successfully!");
    println!("📁 Directory: {}", cli.name);
    println!("🚀 To run: cd {} && cargo run", cli.name);

    Ok(())
}
