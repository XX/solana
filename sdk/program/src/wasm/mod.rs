//! solana-program Javascript interface
#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

pub mod hash;
pub mod instructions;
pub mod pubkey;
pub mod system_instruction;

pub fn display_to_jsvalue<T: std::fmt::Display>(display: T) -> JsValue {
    display.to_string().into()
}
