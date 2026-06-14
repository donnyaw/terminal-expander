use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Match error: {0}")]
    Match(String),

    #[error("Render error: {0}")]
    Render(String),

    #[error("Inject error: {0}")]
    Inject(String),

    #[error("Detect error: {0}")]
    Detect(String),

    #[error("UI error: {0}")]
    Ui(String),
}
