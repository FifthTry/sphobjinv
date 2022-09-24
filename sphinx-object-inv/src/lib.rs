extern crate self as sphinx_object_inv;

mod decoder;
mod error;
mod types;

pub use sphinx_object_inv::error::Error;
pub use sphinx_object_inv::types::{SphinxObjectInv, SphinxObjectInvEntry};

type Result<T> = std::result::Result<T, Error>;
