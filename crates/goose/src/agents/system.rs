use mcp_client::client::Error as ClientError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors from System operation
#[derive(Error, Debug)]
pub enum SystemError {
    #[error("Failed to start the MCP server from configuration `{0}` within 60 seconds")]
    Initialization(SystemConfig),
    #[error("Failed a client call to an MCP server")]
    Client(#[from] ClientError),
    #[error("Transport error: {0}")]
    Transport(#[from] mcp_client::transport::Error),
}

pub type SystemResult<T> = Result<T, SystemError>;

/// Represents the different types of MCP systems that can be added to the manager
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SystemConfig {
    /// Server-sent events client with a URI endpoint
    Sse { uri: String },
    /// Standard I/O client with command and arguments
    Stdio {
        cmd: String,
        args: Vec<String>,
        env: Option<HashMap<String, String>>,
    },
}

impl SystemConfig {
    pub fn sse<S: Into<String>>(uri: S) -> Self {
        Self::Sse { uri: uri.into() }
    }

    pub fn stdio<S: Into<String>>(cmd: S) -> Self {
        Self::Stdio {
            cmd: cmd.into(),
            args: vec![],
            env: None,
        }
    }

    pub fn with_args<I, S>(self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        match self {
            Self::Stdio { cmd, env, .. } => Self::Stdio {
                cmd,
                args: args.into_iter().map(Into::into).collect(),
                env,
            },
            other => other,
        }
    }

    pub fn with_env<I, K, V>(self, env_vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        match self {
            Self::Stdio { cmd, args, .. } => Self::Stdio {
                cmd,
                args,
                env: Some(
                    env_vars
                        .into_iter()
                        .map(|(k, v)| (k.into(), v.into()))
                        .collect(),
                ),
            },
            other => other,
        }
    }
}

impl std::fmt::Display for SystemConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemConfig::Sse { uri } => write!(f, "SSE({})", uri),
            SystemConfig::Stdio { cmd, args, env } => {
                let env_str = env.as_ref().map_or(String::new(), |e| {
                    format!(
                        " with env: {}",
                        e.iter()
                            .map(|(k, v)| format!("{}={}", k, v))
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                });
                write!(f, "Stdio({} {}{})", cmd, args.join(" "), env_str)
            }
        }
    }
}

/// Information about the system used for building prompts
#[derive(Clone, Debug, Serialize)]
pub struct SystemInfo {
    name: String,
    description: String,
    instructions: String,
}

impl SystemInfo {
    pub fn new(name: &str, description: &str, instructions: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            instructions: instructions.to_string(),
        }
    }
}
