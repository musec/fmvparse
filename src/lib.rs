/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

// pub mod boxes;
// pub mod reader;

mod boxes;
mod error;
mod header;
pub mod mp4;

#[macro_use]
extern crate downcast_rs;

pub use error::Error;
pub use header::Header;
