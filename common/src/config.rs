use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub bind_address: String,
    pub webhook_url: String,
    pub github_repo_identifier: String,
    pub unlabeled: Vec<String>,
    pub labels: BTreeMap<String, Vec<String>>,
}

impl Config {
    pub fn from_file_or_default(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path: &Path = path.as_ref();

        if path.exists() {
            let config_source =
                std::fs::read_to_string(path).context("Failed to read configuration.")?;
            let config: Config = serde_yaml::from_str(&config_source)
                .context("Failed to deserialize YAML in configuration.")?;
            Ok(config)
        } else {
            let config = Config::default();
            let yaml = serde_yaml::to_string(&config)
                .context("Failed to serialize YAML for default configuration.")?;
            std::fs::write(path, yaml).context("Failed to write default configuration.")?;
            Ok(config)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:8080".into(),
            webhook_url: "https://discord.com/api/webhooks/blah/blah?wait=true".into(),
            github_repo_identifier: "ModdedMinecraftClub/Mmcc.Bot".into(),
            unlabeled: vec!["@184791836735438858".into()],
            labels: {
                let mut map = BTreeMap::new();
                map.insert(
                    "Needs-Owner-Attention".into(),
                    vec!["@184791836735438858".into(), "@111913367202996224".into()],
                );
                map.insert("Example-Server".into(), vec!["@184791836735438858".into()]);
                map
            },
        }
    }
}
