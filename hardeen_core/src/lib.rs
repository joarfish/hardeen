//! # Hardeen
//!
//! Hardeen is a library which allows for node-based procedural modelling and animation (not yet implemented)
//! of 2d vector graphics.
//! 
//! There are three main components:
//! - GeometryWorld: A datastructure representing points, shapes and groupings of points
//! - Graph: An acyclic, directed graph; each Node with an associated processor
//! - Processors: Processors one or more GeometryWorlds and produce a new one
//! 
//! `handled_vec` provides a datastructure that is used throughout the library. Insted of using smart
//! pointers, Hardeen havily relies on `handles`.


mod geometry;
mod graph;
mod handled_vec;
mod hardeen_error;
mod processors;

pub use crate::graph::*;
pub use crate::processors::*;
pub use crate::geometry::*;
pub use crate::handled_vec::*;
pub use crate::hardeen_error::HardeenError;

pub use crate::graph::ProcessorTypeInfo;