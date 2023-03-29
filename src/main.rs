extern crate structopt;

use rsemulator::EmulatorCore;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

#[derive(StructOpt)]
#[structopt(name = "rust_wasm_sample_parser", about = "Intel 8080 emulator with Rust & WASI.")]
pub struct Options {
    /// The ROM file to run
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

fn main() {
    let options = Options::from_args();
    let contents = fs::read(options.filename)
        .expect("Something went wrong reading the file");
    let emulator = EmulatorCore::new(&contents);
    emulator.run();
}
