use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

// --- Configuration Struct ---

#[derive(Deserialize, Debug, Default)]
struct AppConfig {
    #[serde(default)]
    ignore_keys: Vec<String>,
    #[serde(default)]
    ignore_routes: Vec<String>,
}

// --- HAR Structure Definitions ---

#[derive(Deserialize, Debug)]
struct HarRoot {
    log: HarLog,
}

#[derive(Deserialize, Debug)]
struct HarLog {
    entries: Vec<HarEntry>,
}

#[derive(Deserialize, Debug)]
struct HarEntry {
    request: HarRequest,
    response: HarResponse,
}

#[derive(Deserialize, Debug)]
struct HarRequest {
    method: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct HarResponse {
    content: HarContent,
}

#[derive(Deserialize, Debug)]
struct HarContent {
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
    text: Option<String>,
}

// --- Normalization Engine ---

/// Recursively masks dynamic keys and forces deterministic sorting on all arrays.
fn normalize_json(value: &mut Value, ignore_keys: &[String]) {
    match value {
        Value::Object(map) => {
            // 1. Mask specific keys requested by configuration parameters
            for key in ignore_keys {
                if map.contains_key(key) {
                    map.insert(key.clone(), Value::String("<MASKED>".to_string()));
                }
            }
            // 2. Recurse into remaining properties
            for (_, val) in map.iter_mut() {
                normalize_json(val, ignore_keys);
            }
        }
        Value::Array(vec) => {
            // 1. Recurse into elements first to normalize nested entities
            for val in vec.iter_mut() {
                normalize_json(val, ignore_keys);
            }
            // 2. Deterministically sort the array elements based on stringified format.
            // This normalizes variations caused by non-deterministic database index loops.
            vec.sort_by_key(|v| v.to_string());
        }
        _ => {} // Primitives (Strings, Numbers, Bools, Nulls) remain unchanged
    }
}

/// Transforms dynamic URLs into uniform, file-system safe path strings.
fn parameterize_url(method: &str, url_str: &str) -> String {
    let base_url = url_str.split('?').next().unwrap_or(url_str);

    let uuid_regex =
        Regex::new(r"/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
            .unwrap();
    let numeric_id_regex = Regex::new(r"/\d+").unwrap();

    let parameterized = uuid_regex.replace_all(base_url, "/:uuid");
    let parameterized = numeric_id_regex.replace_all(&parameterized, "/:id");

    let clean_path = parameterized
        .trim_start_matches("https://")
        .trim_start_matches("http://");

    let file_safe = clean_path.replace(['/', ':', '.', ' '], "_");

    format!("{}__{}.json", method.to_uppercase(), file_safe)
}

// --- Configuration Loader ---

fn load_config() -> AppConfig {
    let config_path = Path::new(".hardiffrc");
    if config_path.exists() {
        if let Ok(file) = File::open(config_path) {
            let reader = BufReader::new(file);
            if let Ok(config) = serde_json::from_reader::<_, AppConfig>(reader) {
                println!("Loaded runtime overrides from .hardiffrc");
                return config;
            }
        }
        println!("Warning: .hardiffrc found but could not be parsed. Using defaults.");
    }

    // Default fallbacks if file is absent or corrupted
    AppConfig {
        ignore_keys: vec![
            "timestamp",
            "updated_at",
            "created_at",
            "createdAt",
            "updatedAt",
            "duration_ms",
            "responseTime",
            "sessionId",
            "token",
            "nonce",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        ignore_routes: vec![],
    }
}

// --- File Execution Pipeline ---

fn process_har_file(
    file_path: &Path,
    output_dir: &Path,
    config: &AppConfig,
) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let har: HarRoot = serde_json::from_reader(reader)?;
    let mut written_count = 0;

    for entry in har.log.entries {
        if let Some(ref mime) = entry.response.content.mime_type {
            if mime.contains("application/json") {
                if let Some(ref raw_text) = entry.response.content.text {
                    if let Ok(mut json_value) = serde_json::from_str::<Value>(raw_text) {
                        let filename = parameterize_url(&entry.request.method, &entry.request.url);

                        // Drop processing if the path matches the user blocklist
                        if config.ignore_routes.contains(&filename) {
                            continue;
                        }

                        normalize_json(&mut json_value, &config.ignore_keys);

                        let output_file_path = output_dir.join(filename);
                        let mut out_file = File::create(output_file_path)?;
                        let pretty_json = serde_json::to_string_pretty(&json_value)?;
                        out_file.write_all(pretty_json.as_bytes())?;

                        written_count += 1;
                    }
                }
            }
        }
    }

    Ok(written_count)
}

// --- Execution Harness ---

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Error: Missing operational file targets.");
        println!("Usage: har-diff <source_file.har> <target_file.har>");
        std::process::exit(1);
    }

    let source_har = Path::new(&args[1]);
    let target_har = Path::new(&args[2]);

    // Initialize configuration strategy
    let config = load_config();

    let base_workspace = PathBuf::from(".hardiff/workspace");
    let source_out_dir = base_workspace.join("source");
    let target_out_dir = base_workspace.join("target");

    let _ = fs::remove_dir_all(&base_workspace);
    fs::create_dir_all(&source_out_dir).unwrap();
    fs::create_dir_all(&target_out_dir).unwrap();

    println!("Executing binary extraction pipeline...");

    match process_har_file(source_har, &source_out_dir, &config) {
        Ok(count) => println!("Processed {} baseline source JSON files.", count),
        Err(e) => {
            eprintln!("Critical Error unpacking source HAR: {}", e);
            std::process::exit(1);
        }
    }

    match process_har_file(target_har, &target_out_dir, &config) {
        Ok(count) => println!("Processed {} target migration JSON files.", count),
        Err(e) => {
            eprintln!("Critical Error unpacking target HAR: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n=== Extraction Complete ===");
    println!("Execute the command below to evaluate data-type schema deviations cleanly:");
    println!("git diff --no-index .hardiff/workspace/source .hardiff/workspace/target\n");
}
