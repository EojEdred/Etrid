mod vm;

fn main() {
    match vm::run_wasm() {
        Ok(_) => println!("WASM execution complete."),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}