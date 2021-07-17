//! Library that searches youtube and parses the result to [`model`].
//!
//! [`Reusable`] reuses the same http client on each `GET` request.
//! This takes advantage of keep-alive connections.
#![deny(clippy::inconsistent_struct_constructor)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

mod endpoints;
pub mod error;
pub mod model;
pub(crate) mod parsing;

pub use endpoints::{NotReusable, Reusable};
