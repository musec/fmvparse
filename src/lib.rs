/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

// pub mod boxes;
// pub mod reader;

mod error;
pub mod mp4;
mod boxes;
mod header;

#[macro_use]
extern crate downcast_rs;

pub use error::Error;
pub use header::Header;
