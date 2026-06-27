use crate::config::AppConfig;
use crate::models::HarRoot;
use crate::normalizer::{normalize_json, parameterize_url};

use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

/// Parses an arbitrary HAR trace file, passing payloads through normalization before streaming to target directories.
pub fn process_har_file(
    file_path: &Path,
    output_dir: &Path,
    config: &AppConfig,
) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let har: HarRoot = serde_json::from_reader(reader)?;
    let mut written_count = 0;

    for entry in har.log.entries {
        let mime_valid = entry
            .response
            .content
            .mime_type
            .map(|m| m.contains("application/json"))
            .unwrap_or(false);

        if !mime_valid {
            continue;
        }

        if let Some(raw_text) = entry.response.content.text {
            if let Ok(mut json_value) = serde_json::from_str::<Value>(&raw_text) {
                let filename = parameterize_url(&entry.request.method, &entry.request.url);

                // Early exit if the computed target file matches a user route block
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

    Ok(written_count)
}
