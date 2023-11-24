#![recursion_limit = "128"]
// FIXME: could be worked around in the codegen
#![allow(clippy::large_enum_variant)]
// FIXME: probably fixable with better doc comment formatting, but stripe might also just have doc typos at times that break the regex
#![allow(rustdoc::broken_intra_doc_links)]

mod currency;
mod expandable;
mod generated;
mod ids;
mod pagination;
mod params;

pub use currency::Currency;
pub use expandable::*;
pub use generated::account::AccountId;
pub use generated::application::ApplicationId;
pub use generated::*;
pub use ids::*;
pub use pagination::*;
pub use params::*;

// Allow generated code to use absolute paths starting with `stripe` instead of `crate`
extern crate self as stripe_types;
