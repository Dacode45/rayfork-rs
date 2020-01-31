pub mod core;

/// The raw, unsafe FFI binding, in case you need that escape hatch or the safe layer doesn't provide something you need.
pub mod ffi {
    pub use rayfork_sys::*;
}

pub mod prelude;
