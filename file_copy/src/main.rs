use std::fs;
use std::io::{Read, Write};

fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    let mut input_file =
        fs::File::open(input_fname).map_err(|err| format!("error opening input {}: {}", input_fname, err))?;
    let mut contents = Vec::new();
    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("read error: {}", err))?;

    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
    output_file
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))
}

fn main() {
    let args = vec!["/home/ajayrmavs33236/Desktop/WebAssembly-by-Examples-RustWasm/README.md".to_string(), "/home/ajayrmavs33236/Desktop/WebAssembly-by-Examples-RustWasm/README__.md".to_string()];
    let    mut program = String::new();
    if args.len() > 1 {
        println!("len : {}",args.len());
        program = args[0].clone();
    }

    if args.len() < 2 {
        eprintln!("usage: {} <from> <to>", program);
        return;
    }

    if let Err(err) = process(&args[0], &args[1]) {
        eprintln!("{}", err)
    }
}
