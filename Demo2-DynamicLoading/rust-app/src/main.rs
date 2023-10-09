
use anyhow;
use wasmtime::AsContextMut;
use wasmtime::Engine;
use wasmtime::Instance;
use wasmtime::Linker;
use wasmtime::Module;
use wasmtime::Store;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

struct MyState {
    wasi: WasiCtx,
}

fn main() {
    println!("\r\n>  Initializing WASM engine...");
    let engine: Engine = init_wasm_engine().unwrap();
    println!(">  Loading WASM module...");
    let module: Module = init_wasm_module(&engine).unwrap();
    println!(">  Running WASM function ...");
    let result = wrapper_function(&engine, &module).unwrap();
    println!(">  Result from WASM function: {}\r\n", result);
}

fn init_wasm_engine() -> anyhow::Result<Engine> {
    let engine = Engine::default();
    Ok(engine)
}

fn init_wasm_module(engine: &Engine) -> anyhow::Result<Module> {
    let module = Module::from_file( &engine,
        "../wasm-module/target/wasm32-wasi/release/wasm_module.wasm",
    )?;
    Ok(module)
}

fn wrapper_function(engine: &Engine, module: &Module) -> anyhow::Result<i32> {
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi)?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, MyState { wasi: wasi });
    linker.module(&mut store, "", &module)?;
    let instance: Instance = linker.instantiate(&mut store, &module).unwrap();
    let func_def = instance
        .get_func(&mut store, "sub")
        .expect("function not found !");
    let func_validated = func_def.typed::<(i32, i32), i32>(&store)?;
    let result = func_validated.call(&mut store, (400, 300),)?;
    Ok(result)
}
