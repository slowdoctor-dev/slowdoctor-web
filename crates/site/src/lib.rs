//! Shared site library: data, types, metadata/schema builders, markdown loader,
//! and (added in later phases) page + component views.

pub mod data;
pub mod types;

#[cfg(feature = "ssr")]
pub mod meta;
#[cfg(feature = "ssr")]
pub mod schema;
#[cfg(feature = "ssr")]
pub mod markdown;
