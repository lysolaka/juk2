//! A circular history buffer implementation.

use alloc::string::{String, ToString};

use circular_buffer::CircularBuffer;

/// Circular history buffer of size 16.
///
/// The buffer can also hold the current linebuffer content to save it while browsing history.
pub struct History {
    entries: CircularBuffer<16, String>,
    viewing_entry: Option<usize>,
    saved_line: Option<String>,
}

impl History {
    /// Construct a new [`History`] buffer.
    pub fn new() -> Self {
        Self {
            entries: CircularBuffer::new(),
            viewing_entry: None,
            saved_line: None,
        }
    }

    /// Push `line` to the history.
    ///
    /// If `line` is empty or the same as the previous one, it is not pushed.
    pub fn add(&mut self, line: &str) {
        let line = line.trim();

        if line.is_empty() {
            return;
        }

        if let Some(last) = self.entries.back() {
            if last == line {
                return;
            }
        }

        self.entries.push_back(line.to_string());
        self.viewing_entry = None;
        self.saved_line = None;
    }

    /// Get the previous (older) history entry. 
    ///
    /// Save `current_line` for later, it will be returned when history browsing ends.
    pub fn previous(&mut self, current_line: &str) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        match self.viewing_entry {
            Some(n) => {
                // no overflow will happen since `!self.entries.is_empty()`
                if n < self.entries.len() - 1 {
                    self.viewing_entry = Some(n + 1);
                }
            }
            None => {
                self.saved_line = Some(current_line.to_string());
                self.viewing_entry = Some(0);
            }
        }

        // at this point `self.viewing_entry` is always `Some`
        self.entries
            .nth_back(self.viewing_entry.unwrap())
            .map(|s| s.as_str())
    }

    /// Get the next (more recent) history entry.
    pub fn next(&mut self) -> Option<&str> {
        match self.viewing_entry {
            Some(n) => {
                if n > 0 {
                    self.viewing_entry = Some(n - 1);
                    // `self.viewing_entry` is `Some` as set above
                    self.entries
                        .nth_back(self.viewing_entry.unwrap())
                        .map(|s| s.as_str())
                } else {
                    self.viewing_entry = None;
                    self.saved_line.as_deref()
                }
            }
            None => None,
        }
    }

    /// Reset the history browsing.
    pub fn reset_view(&mut self) {
        self.viewing_entry = None;
    }
}
