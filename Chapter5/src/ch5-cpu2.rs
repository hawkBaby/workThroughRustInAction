struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        //Setup register positions
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        // Combine or then shift left 8 bytes
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow_detected {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }
}

fn main() {
    // Define CPU
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };
    // intialize registers
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;
    // initaliize cpu memory
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x24;
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0],35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0])
}