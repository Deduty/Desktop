use async_std::io::ReadExt;
use async_trait::async_trait;
use deduty_package::DedutyFileReader;
use xresult::{ XReason, XResult };

use super::AutoFileReader;


#[async_trait]
impl DedutyFileReader for AutoFileReader {
    async fn closed(&self) -> XResult<bool> {
        Ok(self.reader.lock().await.is_none())
    }

    async fn close(&self) -> XReason {
        self.reader.lock().await.take();
        Ok(())
    }

    async fn next(&self, chunk: usize) -> XResult<Option<Vec<u8>>> {
        let Some(mut reader) = self.reader.lock().await.take() else {
            return Ok(None);
        };

        let mut buffer = vec![0u8; chunk];
        match reader.read(buffer.as_mut_slice()).await {
            Ok(0) => Ok(None),
            Ok(exact) => {
                self.reader.lock().await.replace(reader);

                buffer.truncate(exact);
                Ok(Some(buffer))
            }
            Err(error) => crate::error::error_err(format!("Unable to read file: {error}"))
        }
    }
}
