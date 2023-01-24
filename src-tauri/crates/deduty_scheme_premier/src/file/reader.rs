use async_std::io::{ BufReader, ReadExt };
use async_std::fs::File;
use async_trait::async_trait;

use deduty_package_traits::DedutyFileReader;
use xresult::{ XError, XReason, XResult };


pub struct PremierFileReader(Option<BufReader<File>>);

impl PremierFileReader {
    pub fn new(file: File) -> Self {
        Self(Some(BufReader::new(file)))
    }
}

#[async_trait]
impl DedutyFileReader for PremierFileReader {
    async fn next(&mut self, chunk: usize) -> XResult<Option<Vec<u8>>> {
        match self.0 {
            Some(ref mut buffer) => {
                let mut external = vec![0u8; chunk];

                match buffer.read(&mut external).await {
                    Ok(0) => {
                        self.0 = None;
                        Ok(None)
                    }
                    Ok(exact) if exact < chunk => {
                        self.0 = None;

                        external.truncate(exact);
                        Ok(Some(external))
                    }
                    Ok(_) => Ok(Some(external)),
                    Err(error) => {
                        self.0 = None;

                        
                        Err(Box::new(XError::from(("FileReaderException", error.to_string()))))
                    }
                }
            }
            None => Err(Box::new(XError::from(("FileReaderException", "Chunked reader is over"))))
        }
    }

    async fn close(&mut self) -> XReason {
        self.0 = None;
        Ok(())
    }

    fn closed(&self) -> bool {
        self.0.is_none()
    }
}
