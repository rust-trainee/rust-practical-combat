struct CPU {
    registers: [u8; 16],
    // 使用 usize 而不是 u16 与最初的规范是不同的, 但我们使用 usize, 是因为 Rust
    // 允许此类型被用于索引经济
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // 要创建一个 u16
        // 类型的操作吗,我们使用逻辑或操作,把内存中的两个值合并到一起.这两个值需要先转换为
        // u16,如果不先做这个转换,左移位会将所有的位数都设置为0
        (op_byte1 << 8) | op_byte2
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        // 对于 u8 类型来说, overflow_add 方法的返回类型为(u9, bool)
        // 如果检测到溢出则返回值这个布尔类型的值为 true
        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xf] = 0;
        }
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
            };
        }
    }
}
fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    // 加法操作 0x8014, 将寄存器1 的值加到寄存器 0 上
    mem[0] = 0x80;
    mem[1] = 0x14;

    // 加法操作 0x8024, 将寄存器 2 的值加到寄存器0 上
    mem[2] = 0x80;
    mem[3] = 0x24;

    // 加法操作 0x8034, 将寄存器 3 的值 加到寄存器 0 上
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
