//! Wasm 智能合约虚拟机 - 链上代码执行沙箱
use wasmtime::*;

pub struct WasmContractVM {
    engine: Engine,
    store: Store<()>,
}

impl WasmContractVM {
    pub fn new() -> Self {
        let config = Config::default();
        let engine = Engine::new(&config).unwrap();
        let store = Store::new(&engine, ());
        WasmContractVM { engine, store }
    }

    pub fn execute_contract(&self, bytecode: &[u8], input: i32) -> i32 {
        let module = Module::new(&self.engine, bytecode).unwrap();
        let mut linker = Linker::new(&self.engine);
        let instance = linker.instantiate(&self.store, &module).unwrap();
        let func = instance.get_typed_func::<i32, i32>(&self.store, "run").unwrap();
        func.call(&self.store, input).unwrap()
    }
}
