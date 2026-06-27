use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub ignore_keys: Vec<String>,
    pub ignore_routes: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
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
}

impl AppConfig {
    /// Discovers and parses a `.hardiffrc` file, falling back cleanly to defaults if missing.
    pub fn load_from_env() -> Self {
        let config_path = Path::new(".hardiffrc");

        if !config_path.exists() {
            return Self::default();
        }

        match File::open(config_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader::<_, AppConfig>(reader) {
                    Ok(config) => {
                        println!("Loaded execution runtime overrides from .hardiffrc");
                        config
                    }
                    Err(_) => {
                        eprintln!(
                            "Warning: .hardiffrc found but corrupted. Falling back to defaults."
                        );
                        Self::default()
                    }
                }
            }
            Err(_) => Self::default(),
        }
    }
}
