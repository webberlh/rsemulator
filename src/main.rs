extern crate structopt;
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

pub struct EmulatorCore<'a> {
    rom: &'a str,
}

impl EmulatorCore<'_> {
    fn new(rom: &str) -> EmulatorCore {
        EmulatorCore { rom: rom }
    }
    fn run(self) -> () {
        todo!()
    }
}

fn main() {
    let options = Options::from_args();
    let contents = fs::read_to_string(options.filename)
        .expect("Something went wrong reading the file");
    let emulator = EmulatorCore::new(&contents);
    emulator.run();
}
