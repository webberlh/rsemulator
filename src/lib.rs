pub struct EmulatorCore<'a> {
    rom: &'a Vec<u8>,
    pc: usize,
}

impl EmulatorCore<'_> {
    pub fn new(rom: &Vec<u8>) -> EmulatorCore {
        EmulatorCore { rom: rom, pc: 0 }
    }
    pub fn run(self) -> () {
        println!("{}", self.rom[self.pc])
    }
}
