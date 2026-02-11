//! A linebuffer implementation with support for UTF-8.

use alloc::string::String;
use core::mem;

use str_indices::chars;

/// A linebuffer implementation supporting UTF-8 operations.
///
/// Designed to work with [`crate::Interface`].
///
/// The backing storage of the buffer is [`String`], with capacity of 128 as default.
pub struct LineBuffer {
    buf: String,
    cursor_pos: usize,
}

impl LineBuffer {
    /// Construct a new [`LineBuffer`].
    pub fn new() -> Self {
        Self {
            buf: String::with_capacity(128),
            cursor_pos: 0,
        }
    }

    /// Clear the [`LineBuffer`] and shrink its allocation to the default size.
    pub fn clear(&mut self) {
        self.buf.clear();
        self.buf.shrink_to(128);
        self.cursor_pos = 0;
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Cursor position as the [`char`] count from the index 0.
    pub fn cursor_char_pos(&self) -> usize {
        chars::count(&self.buf[..self.cursor_pos])
    }

    /// Cursor position as the byte position in the string.
    ///
    /// Note that by design this position always lies on a [`char`] boundary.
    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    /// Buffer length in [`char`] units.
    pub fn len(&self) -> usize {
        chars::count(&self.buf)
    }

    /// Borrows the inner buffer as is.
    pub fn as_str(&self) -> &str {
        &self.buf
    }

    /// Take the contents of the line buffer, leaving it empty.
    ///
    /// This function also strips the resulting string before returning it.
    ///
    /// # Warning
    ///
    /// Since the resulting string is stripped, the position returned by 
    /// [`LineBuffer::cursor_pos()`] or [`LineBuffer::cursor_char_pos()`] is not valid for it.
    pub fn take(&mut self) -> String {
        // strip in place, adapted from: https://docs.rs/string_more/latest/src/string_more/lib.rs.html#524
        let trimmed = self.buf.trim();
        let len = trimmed.len();

        // SAFETY: since we are using `ptr::offset_from()` to compute the length of a slice, we are
        // OK as the docs say.
        let start = unsafe { trimmed.as_ptr().offset_from(self.buf.as_ptr()) } as usize;

        // SAFETY: modifications on the `&mut Vec<u8>` keep it valid UTF-8: we are copying a
        // UTF-8 slice from further on in the string.
        unsafe { self.buf.as_mut_vec().copy_within(start..start + len, 0) };

        self.buf.truncate(len);
        // take the old string
        mem::replace(&mut self.buf, String::with_capacity(128))
    }

    /// Insert a character at the cursor's position.
    pub fn insert_char(&mut self, c: char) {
        self.buf.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
    }

    /// Delete a character before the cursor. (Backspace)
    ///
    /// Returns `true` if a character was deleted, `false` if the cursor is at the start.
    pub fn delete_before_cursor(&mut self) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos = self.buf.floor_char_boundary(self.cursor_pos - 1);
            self.buf.remove(self.cursor_pos);
            true
        } else {
            false
        }
    }

    /// Delete a character at the cursor. (Delete)
    ///
    /// Returns `true` if a character was deleted, `false` if the cursor is at the end.
    pub fn delete_at_cursor(&mut self) -> bool {
        if self.cursor_pos < self.buf.len() {
            self.buf.remove(self.cursor_pos);
            true
        } else {
            false
        }
    }

    /// Moves the cursor once to the left.
    ///
    /// Returns `true` if the cursor moved, `false` if already at the start.
    pub fn move_cursor_left(&mut self) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos = self.buf.floor_char_boundary(self.cursor_pos - 1);
            true
        } else {
            false
        }
    }

    /// Moves the cursor once to the right.
    ///
    /// Returns `true` if the cursor moved, `false` if already at the end.
    pub fn move_cursor_right(&mut self) -> bool {
        if self.cursor_pos < self.buf.len() {
            self.cursor_pos = self.buf.ceil_char_boundary(self.cursor_pos + 1);
            true
        } else {
            false
        }
    }

    /// Moves the cursor to the start of the buffer.
    ///
    /// Returns the number of positions the cursor moved.
    pub fn move_cursor_to_start(&mut self) -> usize {
        let old_pos = self.cursor_pos;
        self.cursor_pos = 0;
        chars::count(&self.buf[..old_pos])
    }

    /// Moves the cursor to the end of the buffer.
    ///
    /// Returns the number of positions the cursor moved.
    pub fn move_cursor_to_end(&mut self) -> usize {
        let old_pos = self.cursor_pos;
        self.cursor_pos = self.buf.len();
        chars::count(&self.buf[old_pos..])
    }

    /// Returns the byte position of the left word's start
    fn find_word_start_left(&self) -> usize {
        if self.cursor_pos == 0 {
            return 0;
        }

        let mut iter = self.buf[..self.cursor_pos].char_indices().rev().peekable();

        // skip whitespace
        while let Some(&(_, ch)) = iter.peek() {
            if !ch.is_whitespace() {
                break;
            }
            iter.next();
        }

        let (mut start, ch) = match iter.next() {
            Some(v) => v,
            None => return 0,
        };

        if Self::is_ident_char(ch) {
            // walk over word chars
            while let Some(&(pos, ch)) = iter.peek() {
                if !Self::is_ident_char(ch) {
                    break;
                }
                start = pos;
                iter.next();
            }
        }

        start
    }

    /// Returns the byte position of the right word's end
    fn find_word_end_right(&self) -> usize {
        if self.cursor_pos >= self.buf.len() {
            return self.buf.len();
        }

        let mut iter = self.buf[self.cursor_pos..].char_indices().peekable();

        // skip whitespace
        while let Some(&(_, ch)) = iter.peek() {
            if !ch.is_whitespace() {
                break;
            }
            iter.next();
        }

        let (rel_start, ch) = match iter.next() {
            Some(v) => v,
            None => return self.buf.len(),
        };

        let mut end = rel_start + ch.len_utf8();

        if Self::is_ident_char(ch) {
            // walk over current word's chars
            while let Some(&(pos, ch)) = iter.peek() {
                if !Self::is_ident_char(ch) {
                    break;
                }
                end = pos + ch.len_utf8();
                iter.next();
            }
        }

        self.cursor_pos + end
    }

    /// Moves the cursor to the start of the previous word.
    ///
    /// Words are defined as sequences of alphanumeric characters and underscores.
    /// Symbols (like `+`, `-`, `*`) are treated as separate words. Only whitespace
    /// is skipped when navigating between words.
    ///
    /// Returns the number of positions the cursor moved.
    pub fn move_cursor_word_left(&mut self) -> usize {
        let old = self.cursor_pos;
        let new = self.find_word_start_left();
        self.cursor_pos = new;

        chars::count(&self.buf[new..old])
    }

    /// Moves the cursor to the end of the current or the next word if on a whitespace.
    ///
    /// Words are defined as sequences of alphanumeric characters and underscores.
    /// Symbols (like `+`, `-`, `*`) are treated as separate words. Only whitespace
    /// is skipped when navigating between words.
    ///
    /// Returns the number of positions the cursor moved.
    pub fn move_cursor_word_right(&mut self) -> usize {
        let old = self.cursor_pos;
        let new = self.find_word_end_right();
        self.cursor_pos = new;

        chars::count(&self.buf[old..new])
    }

    /// Deletes the word to the left of the cursor (CTRL + Backspace).
    ///
    /// Returns the number of [`char`]s deleted.
    pub fn delete_word_left(&mut self) -> usize {
        let start = self.find_word_start_left();
        let end = self.cursor_pos;

        if start == end {
            return 0;
        }

        let deleted = chars::count(&self.buf[start..end]);
        self.buf.replace_range(start..end, "");
        self.cursor_pos = start;

        deleted
    }

    /// Deletes the word to the right of the cursor (CTRL + Delete).
    ///
    /// Returns the number of [`char`]s deleted.
    pub fn delete_word_right(&mut self) -> usize {
        let start = self.cursor_pos;
        let end = self.find_word_end_right();

        if start == end {
            return 0;
        }

        let deleted = chars::count(&self.buf[start..end]);
        self.buf.replace_range(start..end, "");

        deleted
    }

    /// Loads text into the buffer, replacing existing content.
    ///
    /// The cursor is positioned at the end of the loaded text.
    ///
    /// Used for history navigation.
    pub fn load(&mut self, text: &str) {
        self.buf.clear();
        self.buf.push_str(text);
        self.cursor_pos = self.buf.len();
    }

    /// Predicate function used to determine if `c` is part of a word (identifier).
    #[inline]
    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
}
