#![feature(trait_upcasting)]

use deduty_service::Service;

mod reader;
mod file;
mod lection;

mod error;
mod borrowed;

mod package;
mod service;
mod schemes;

pub fn service() -> Box<dyn Service> {
    Box::<service::AutoPackageService>::default() as Box<dyn Service>
}
