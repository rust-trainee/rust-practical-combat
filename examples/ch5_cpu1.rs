/// # 实现一个 CPU 模拟器
///
///
/// ## 加法器
/// ### 术语
/// - 操作: 只该系统本身支持的过程
/// - 寄存器: 一种数据的容器,CPU 能够直接访问保存在启动的数据. 每个寄存器都是一个 u8
/// - 操作码: 一个数字,对应某个具体的操作. 在
/// CHIP-8平台上,操作码既包含具体的操作,也包含用于保存操作数的寄存器.

#[derive(Debug)]
struct CPU {
    // CHIP-8 的所有操作码都是 u16 类型的.
    current_operation: u16,
    // 对加法运算来说,这两个寄存器都足够了
    registers: [u8; 2],
}

impl CPU {
    /// 读取操作码
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }
    fn run(&mut self) {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8; // 提取寄存器数量
        let x = ((opcode & 0x0F00) >> 8) as u8; // 提取第一个寄存器编号
        let y = ((opcode & 0x00F0) >> 4) as u8; // 提取第二个寄存器编号
        let d = ((opcode & 0b1111) >> 0) as u8; // 提取操作符

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    // 8:表示此操作需要两个寄存器
    // 0:表示0 号寄存器
    // 1:表示1 号寄存器
    // 4:表示此操作是加法运算.
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.run();
    assert_eq!(cpu.registers[0], 15);
    println!("cpu: {:?}", cpu);
}
