mod data_type;
mod storage_manager;
pub use data_type::*;
pub use storage_manager::*;

// auto-publib crate-only exclude=[data_type,storage_manager]
pub(crate) mod _unit_testing;

mod data_category;
mod data_named;

pub(crate) use data_category::*;
pub(crate) use data_named::*;