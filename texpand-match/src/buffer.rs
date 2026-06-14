#[derive(Debug, Clone)]
pub struct RollingBuffer {
    buffer: String,
    max_len: usize,
}

impl RollingBuffer {
    pub fn new(max_len: usize) -> Self {
        Self {
            buffer: String::new(),
            max_len,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
        if self.buffer.len() > self.max_len {
            self.buffer.drain(..self.buffer.len() - self.max_len);
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn content(&self) -> &str {
        &self.buffer
    }

    pub fn ends_with(&self, s: &str) -> bool {
        self.buffer.ends_with(s)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
