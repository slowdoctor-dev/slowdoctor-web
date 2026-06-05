//! Shared site library: data, types, metadata/schema builders, markdown loader,
//! and (added in later phases) page + component views.

pub mod data;
#[cfg(feature = "ssr")]
pub mod dates;
#[cfg(feature = "ssr")]
pub mod frontmatter;
pub mod types;

#[cfg(any(feature = "ssr", feature = "csr"))]
pub mod components;

#[cfg(feature = "ssr")]
pub mod markdown;
#[cfg(feature = "ssr")]
pub mod meta;
#[cfg(feature = "ssr")]
pub mod pages;
#[cfg(feature = "ssr")]
pub mod schema;
