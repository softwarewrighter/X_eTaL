//! Whether the text changed since it was loaded or saved; the text
//! replaced (a fix made); the line-end motion; an empty buffer by default.

use crate::Buffer;

impl Buffer {
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Replace the whole text (an edit, so the buffer is dirty), the
    /// cursor back at the start.
    pub fn replace(&mut self, text: &str) {
        *self = Buffer {
            dirty: true,
            ..Buffer::new(text)
        };
    }

    pub fn end(&mut self) {
        self.col = self.width(self.row);
    }
}

impl Default for Buffer {
    /// An empty buffer.
    fn default() -> Self {
        Buffer::new("")
    }
}
