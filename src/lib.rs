//! Binary Space Partitioning tree for 2D space.
//!
//! Provides BSP tree construction, front-back ordering, portal generation,
//! and potentially visible set (PVS) computation.

pub mod bsp;
pub mod node;
pub mod portal;
pub mod pvs;
pub mod splitter;

pub use bsp::BspTree;
pub use node::BspNode;
pub use portal::{Portal, generate_portals};
pub use pvs::compute_pvs;
pub use splitter::split_polygon;
