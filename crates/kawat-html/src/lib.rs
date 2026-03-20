//! HTML tree cleaning, tag normalization, and link density calculation.
//!
//! Mirrors trafilatura's `htmlprocessing.py` + `settings.py` tag lists.

pub mod clean;
pub mod convert;
pub mod density;
pub mod tags;

pub use clean::tree_cleaning;
pub use convert::convert_tags;
pub use density::link_density_test;
pub use tags::{MANUALLY_CLEANED, MANUALLY_STRIPPED, TAG_CATALOG};
