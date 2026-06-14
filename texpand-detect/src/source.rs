use crate::InputEvent;

#[derive(Default)]
pub struct SourceConfig {
    pub device_paths: Vec<String>,
}

pub trait KeySource: Send {
    fn initialize(&mut self) -> anyhow::Result<()>;
    fn read_event(&mut self) -> anyhow::Result<Option<InputEvent>>;
}
