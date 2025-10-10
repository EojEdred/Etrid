use wasmi::{Engine, Module, Store, Linker, Extern, TypedFunc};
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run_wasm() -> Result<(), Box<dyn Error>> {
    let wasm_path = Path::new("contracts/dummy.wasm");
    let wasm_binary = fs::read(&wasm_path)
        .map_err(|_| format!("Failed to read WASM file at {:?}", wasm_path))?;

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_binary)?;

    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);

    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;

    if let Some(Extern::Func(func)) = instance.get_export(&store, "main") {
        let typed: TypedFunc<(), ()> = func.typed(&store)?;
        typed.call(&mut store, ())?;
        println!("Executed WASM `main()` function successfully.");
    } else {
        println!("No `main` function found in WASM.");
    }

    Ok(())
}