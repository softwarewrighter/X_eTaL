//! Whether the text changed since it was loaded or saved; and the
//! line-end motion.

use crate::Buffer;

impl Buffer {
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    pub fn end(&mut self) {
        self.col = self.width(self.row);
    }
}
