pub mod config;
pub mod error;
pub mod match_file;

pub use config::{Config, FieldConfig, MatchFile, VariableDef};
pub use error::Error as ConfigError;
