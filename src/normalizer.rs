use regex::Regex;
use serde_json::Value;
use std::sync::OnceLock;

/// Compiles and caches the UUID regular expression pattern exactly once for the process lifetime.
fn uuid_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
            .unwrap()
    })
}

/// Compiles and caches the Numeric ID regular expression pattern exactly once for the process lifetime.
fn numeric_id_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"/\d+").unwrap())
}

/// Recursively scans JSON objects to mask targeted dynamic keys and enforce deterministic array ordering.
pub fn normalize_json(value: &mut Value, ignore_keys: &[String]) {
    match value {
        Value::Object(map) => {
            // Mask keys configured by the evaluation context
            for key in ignore_keys {
                if map.contains_key(key) {
                    map.insert(key.clone(), Value::String("<MASKED>".to_string()));
                }
            }
            // Recurse down remaining child entries
            for (_, val) in map.iter_mut() {
                normalize_json(val, ignore_keys);
            }
        }
        Value::Array(vec) => {
            // Normalize inner values first to catch nested deviations
            for val in vec.iter_mut() {
                normalize_json(val, ignore_keys);
            }
            // Sort array items by their serialized string representations
            // to neutralize database extraction entropy.
            vec.sort_by_key(|v| v.to_string());
        }
        _ => {} // Primitive configurations pass unaltered
    }
}

/// Transforms dynamic query-laden URLs into flat, clean file-system compatible filenames.
pub fn parameterize_url(method: &str, url_str: &str) -> String {
    let base_url = url_str.split('?').next().unwrap_or(url_str);

    // Apply zero-overhead cached regular expression translations
    let parameterized = uuid_regex().replace_all(base_url, "/:uuid");
    let parameterized = numeric_id_regex().replace_all(&parameterized, "/:id");

    let clean_path = parameterized
        .trim_start_matches("https://")
        .trim_start_matches("http://");

    let file_safe = clean_path.replace(['/', ':', '.', ' '], "_");

    format!("{}__{}.json", method.to_uppercase(), file_safe)
}
