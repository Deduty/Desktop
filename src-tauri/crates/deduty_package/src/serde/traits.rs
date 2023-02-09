use async_trait::async_trait;
use serde::Serialize;
use xresult::XResult;


#[async_trait]
pub trait AsyncTrySerialize<T: Serialize> {
    async fn try_serde(&self) -> XResult<T>;
}
