use esp_hal::uart::{IoError, Uart};

/// Terminal trait used to implement the REPL interface.
#[allow(async_fn_in_trait)]
pub trait Terminal {
    type Error;

    /// Read a single byte from the input.
    async fn read_byte(&mut self) -> Result<u8, Self::Error>;
    /// Write the entire contents of `buf` to the output sink.
    async fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
    /// Flush the output sink.
    async fn flush(&mut self) -> Result<(), Self::Error>;

    async fn cursor_left(&mut self) -> Result<(), Self::Error> {
        self.write(b"\x1b[D").await
    }

    async fn cursor_right(&mut self) -> Result<(), Self::Error> {
        self.write(b"\x1b[C").await
    }

    async fn clear_eol(&mut self) -> Result<(), Self::Error> {
        self.write(b"\x1b[K").await
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
