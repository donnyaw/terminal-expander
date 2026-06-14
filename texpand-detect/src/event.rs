#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress { code: u16, key: String },
    KeyRelease { code: u16, key: String },
    Unknown { code: u16, value: i32 },
}
