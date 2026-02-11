use esp_hal::uart::{IoError, Uart};

/// Terminal trait used to implement the REPL interface.
///
/// This trait should be implemented on types, which perform user I/O.
#[allow(async_fn_in_trait)]
pub trait Terminal {
    type Error;

    /// Read a single byte from the input source.
    ///
    /// If the input buffer is empty the implementation should asynchronously wait for a byte to
    /// become available.
    async fn read_byte(&mut self) -> Result<u8, Self::Error>;
    /// Write the entire contents of `buf` to the output sink.
    ///
    /// In standard Rust terminology this function should be named `write_all()`
    async fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
    /// Flush the output sink.
    ///
    /// The implementation should make sure that all pending data is transmitted.
    async fn flush(&mut self) -> Result<(), Self::Error>;

    /// Move the terminal cursor left.
    ///
    /// The default implementation uses an ANSI escape sequence `<ESC>[D`. An implementation could
    /// call a platform API instead.
    async fn cursor_left(&mut self) -> Result<(), Self::Error> {
        self.write(b"\x1b[D").await
    }

    /// Move the terminal cursor right.
    ///
    /// The default implementation uses an ANSI escape sequence `<ESC>[C`. An implementation could
    /// call a platform API instead.
    async fn cursor_right(&mut self) -> Result<(), Self::Error> {
        self.write(b"\x1b[C").await
    }

    /// Clear text from the cursor to the end of the line.
    ///
    /// The default implementation uses an ANSI escape sequence `<ESC>[0K`. An implementation could
    /// call a platform API instead.
    async fn clear_eol(&mut self) -> Result<(), Self::Error> {
        // <ESC>[0K is more explicit about what is happening, even though missing parameters
        // default to 0
        self.write(b"\x1b[0K").await
    }
}

impl<'d> Terminal for Uart<'d, esp_hal::Async> {
    type Error = IoError;

    async fn read_byte(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0; 1];
        self.read_exact_async(&mut buf).await?;
        Ok(buf[0])
    }

    async fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        let mut n = 0;
        while n < buf.len() {
            n += self.write_async(&buf[n..]).await?;
        }
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(self.flush_async().await?)
    }
}
