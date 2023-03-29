pub struct EmulatorCore {
    pc: usize,
    cycles: usize,
    b: u16,
    acc: u8,
}

enum EmulatorOPCode {
    NOOP,
    LXIBd16(u16),
    STAXB,
}

impl EmulatorCore {
    // pub fn new(rom: &Vec<u8>) -> EmulatorCore {
    //     EmulatorCore { rom: rom, pc: 0 }
    // }
    pub fn run(rom: &Vec<u8>) -> () {
        let mut stack: Vec<u8> = vec![0; 1<<16];
        let mut emulator = EmulatorCore {pc: 0, cycles: 0, b: 0, acc: 0};
        while emulator.pc < rom.len() {
            emulator = EmulatorCore::do_op(&emulator, &mut stack, rom);
        }
    }

    fn construct_op(pc: usize, rom: &Vec<u8>) -> EmulatorOPCode {
        match rom[pc] {
            0x00 => EmulatorOPCode::NOOP,
            0x01 => EmulatorOPCode::LXIBd16(((rom[pc+1] as u16) << 8) | rom[pc+2] as u16),
            0x02 => EmulatorOPCode::STAXB,
            _ => todo!(),
        }
    }

    fn do_op(emulator: &EmulatorCore, stack: &mut Vec<u8>, rom: &Vec<u8>) -> EmulatorCore {
        match EmulatorCore::construct_op(emulator.pc, rom) {
            EmulatorOPCode::NOOP => EmulatorCore { pc: emulator.pc + 1, cycles: emulator.cycles + 4, b: emulator.b, acc: emulator.acc},
            EmulatorOPCode::LXIBd16(val) => EmulatorCore { pc: emulator.pc + 3, cycles: emulator.cycles + 10, b: val, acc: emulator.acc},
            EmulatorOPCode::STAXB => {
                stack[emulator.b as usize] = emulator.acc;
                EmulatorCore { pc: emulator.pc + 1, cycles: emulator.cycles + 7, b: emulator.b, acc: emulator.acc}
            }
        }
    }
}
