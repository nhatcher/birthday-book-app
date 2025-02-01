#[cfg(not(target_arch = "wasm32"))]
mod not_wasm32;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use not_wasm32::{get_timestamp, uuid4};

#[cfg(target_arch = "wasm32")]
mod wasm32;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm32::{get_timestamp, uuid4};
