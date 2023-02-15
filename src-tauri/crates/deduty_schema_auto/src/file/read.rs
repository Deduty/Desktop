use async_std::fs::File;
use async_trait::async_trait;
use deduty_package::{ DedutyFileReader, ReadFile };
use xresult::XResult;

use crate::reader::AutoFileReader;

use super::AutoFile;


#[async_trait]
impl ReadFile for AutoFile {
    async fn reader(&self) -> XResult<Box<dyn DedutyFileReader>> {
        let file = File::open(self.path.clone())
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to open file `{:#?}`: {error}", self.path)))?;

        Ok(Box::new(AutoFileReader::new(file)) as Box<dyn DedutyFileReader>)
    }
}
