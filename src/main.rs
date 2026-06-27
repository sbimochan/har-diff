mod config;
mod models;
mod normalizer;
mod pipeline;

use config::AppConfig;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Error: Missing operational file paths.");
        eprintln!("Usage: har-diff <source_file.har> <target_file.har>");
        std::process::exit(1);
    }

    let source_har = Path::new(&args[1]);
    let target_har = Path::new(&args[2]);

    // Bootstrap configuration matrix
    let config = AppConfig::load_from_env();

    // Enforce isolated working directory architectures
    let base_workspace = PathBuf::from(".hardiff/workspace");
    let source_out_dir = base_workspace.join("source");
    let target_out_dir = base_workspace.join("target");

    // Clear and re-initialize target workspaces
    let _ = fs::remove_dir_all(&base_workspace);
    fs::create_dir_all(&source_out_dir).expect("Failed to initialize source workspace directory.");
    fs::create_dir_all(&target_out_dir).expect("Failed to initialize target workspace directory.");

    println!("Starting modular contract processing execution pipeline...");

    match pipeline::process_har_file(source_har, &source_out_dir, &config) {
        Ok(count) => println!(
            "Successfully materialized {} baseline source entries.",
            count
        ),
        Err(e) => {
            eprintln!(
                "Critical infrastructure pipeline error on Source data: {}",
                e
            );
            std::process::exit(1);
        }
    }

    match pipeline::process_har_file(target_har, &target_out_dir, &config) {
        Ok(count) => println!(
            "Successfully materialized {} migration target entries.",
            count
        ),
        Err(e) => {
            eprintln!(
                "Critical infrastructure pipeline error on Target data: {}",
                e
            );
            std::process::exit(1);
        }
    }

    println!("\n=== Extraction Phase Complete ===");
    println!("Execute the terminal tracking routine below to evaluate mutations:");
    println!("git diff --no-index .hardiff/workspace/source .hardiff/workspace/target\n");
}
