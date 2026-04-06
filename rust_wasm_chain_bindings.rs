//! Rust-Wasm 链绑定 - Web 前端区块链交互
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmChain { height: u64 }

#[wasm_bindgen]
impl WasmChain {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self { WasmChain { height: 0 } }

    pub fn add_block(&mut self) { self.height += 1; }
    pub fn get_height(&self) -> u64 { self.height }
}
