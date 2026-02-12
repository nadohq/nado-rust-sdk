use crate::utils::deployment::Deployment;
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;

pub static CONFIGS: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/nado_utils/configs");
#[derive(Clone, PartialEq)]
pub enum ClientMode {
    Test,
    Prod,
    Local,
    LocalAlt,
}

impl ClientMode {
    fn get_domain() -> &'static str {
        let use_backup = std::env::var("USE_BACKUP_PUBLIC_BACKEND")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";
        if use_backup {
            "nado-backend.xyz"
        } else {
            "nado.xyz"
        }
    }

    pub fn default_gateway_url(&self) -> String {
        let envtag = self.nado_envtag();
        let domain = Self::get_domain();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://gateway.{envtag}.{domain}:80/v1")
            }
            _ => {
                format!("https://gateway.{envtag}.{domain}/v1")
            }
        }
    }

    pub fn default_trigger_url(&self) -> String {
        let envtag = self.nado_envtag();
        let domain = Self::get_domain();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://trigger.{envtag}.{domain}:8080/v1")
            }
            _ => {
                format!("https://trigger.{envtag}.{domain}/v1")
            }
        }
    }

    pub fn default_archive_url(&self) -> String {
        let envtag = self.nado_envtag();
        let domain = Self::get_domain();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://archive.{envtag}.{domain}:8000/v1")
            }
            _ => {
                format!("https://archive.{envtag}.{domain}/v1")
            }
        }
    }

    pub fn default_gateway_ws_url(&self) -> String {
        let envtag = self.nado_envtag();
        let domain = Self::get_domain();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("ws://gateway.{envtag}.{domain}:80/ws")
            }
            _ => {
                format!("wss://gateway.{envtag}.{domain}/ws")
            }
        }
    }

    pub fn private_gateway_ws_url(&self) -> String {
        let envtag = self.nado_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("ws://{envtag}-mm.nado-backend.xyz:80/ws")
            }
            _ => {
                format!("wss://{envtag}-mm.nado-backend.xyz/ws")
            }
        }
    }

    pub fn private_gateway_subscription_url(&self) -> String {
        let envtag = self.nado_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("ws://{envtag}-mm.nado-backend.xyz:80/subscribe")
            }
            _ => {
                format!("wss://{envtag}-mm.nado-backend.xyz/subscribe")
            }
        }
    }

    pub fn private_gateway_url(&self) -> String {
        let envtag = self.nado_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://{envtag}-mm.nado-backend.xyz:80")
            }
            _ => {
                format!("https://{envtag}-mm.nado-backend.xyz")
            }
        }
    }

    pub fn nado_envtag(&self) -> String {
        match self {
            Self::Test => "test",
            Self::Prod => "prod",
            Self::Local => "local",
            Self::LocalAlt => "local-alt",
        }
        .to_string()
    }

    pub fn from_envtag(envtag: &str) -> Self {
        match envtag {
            "test" => Self::Test,
            "prod" => Self::Prod,
            "local" => Self::Local,
            "local-alt" => Self::LocalAlt,
            _ => panic!("Unknown envtag: {envtag}"),
        }
    }

    pub fn deployment(&self) -> Deployment {
        let network = self.nado_envtag();
        let deployment_path = Path::new(&network).join("deployment.json");

        // Order of loading
        // 1. try ENVVAR override
        // 2. try locally configured directory
        // 3. use deployment.json included in binary
        let deployment_str = match std::env::var("NADO_CONFIGS_DIR") {
            Ok(path) => {
                let deployment_path = Path::new(&path).join(&network).join("deployment.json");
                fs::read_to_string(&deployment_path).unwrap_or_else(|_| {
                    panic!("Failed to read {}", &deployment_path.to_string_lossy())
                })
            }
            Err(_) => {
                let package_dir = env!("CARGO_MANIFEST_DIR");
                let default_configs_path = Path::new(package_dir)
                    .join("src/nado_utils/configs")
                    .join(&network)
                    .join("deployment.json");

                fs::read_to_string(&default_configs_path).unwrap_or_else(|_| {
                    CONFIGS
                        .get_file(deployment_path.to_str().unwrap())
                        .expect("Deployment file not found")
                        .contents_utf8()
                        .expect("Failed to read deployment file")
                        .to_string()
                })
            }
        };

        serde_json::from_str(&deployment_str).unwrap()
    }
}
