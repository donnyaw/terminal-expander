pub mod config;
pub mod csv;
pub mod error;
pub mod match_file;

pub use config::{Config, FieldConfig, MatchFile, VariableDef};
pub use csv::{
    merge_records, read_csv_file, records_to_csv, records_to_json, write_csv_file, TriggerRecord,
};
pub use error::Error as ConfigError;
