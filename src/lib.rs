//! WAFFLE Wasm analysis framework.

#![allow(dead_code)]

// Re-export wasmparser and wasmencoder for easier use of the right
// version by our embedders.
pub use wasm_encoder;
pub use wasmparser;

mod analysis;
mod backend;
mod cfg;
mod frontend;
mod ir;
mod op_traits;
mod pass;

pub use ir::*;
