mod static_mesh_storage;
pub(crate) use static_mesh_storage::*;

// auto-publib exclude=[static_mesh_storage]
pub mod lenses;
pub mod model;

mod entity;
mod lens;
mod scene;

pub use entity::*;
pub use lens::*;
pub use scene::*;