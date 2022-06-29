use std::{
    env,
    fs::File,
    io::{self, Read},
};

/// Default DRAM size (128MiB).
pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

#[derive(Debug)]
struct Cpu {
    regs: [u64; 32],
    pc: u64,
    dram: Vec<u8>,
}

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
        let mut cpu = Self {
            regs: [0; 32],
            pc: 0,
            dram: code,
        };
        cpu.regs[2] = DRAM_SIZE;
        cpu
    }

    fn fetch(&self) -> u32 {
        // Read 32-bit instruction from a memory.
        // little endian
        let index = self.pc as usize;
        // u8 * 4 = u32
        // risc-v指令是小端序，且是定长的32位指令
        // 因此右边的部分需要左移，然后将4个u8拼在一起
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    fn execute(&mut self, inst: u32) {
        // 解码
        // 低6位是opcode
        let opcode = inst & 0x7f;
        // R型
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
            }
        }
    }

    // Print values in all registers (x0-x31).
    pub fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:<#18x} x{:02}({})={:<#18x} x{:02}({})={:<#18x} x{:02}({})={:<#18x}",
                    i,
                    abi[i],
                    self.regs[i],
                    i + 1,
                    abi[i + 1],
                    self.regs[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.regs[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: riscv-emulator <filename>")
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    while cpu.pc < cpu.dram.len() as u64 {
        // 1. fetch instruction
        let inst = cpu.fetch();

        // 2. add pc
        cpu.pc += 4;
        // 3. Decode & Execute
        cpu.execute(inst);
    }
    cpu.dump_registers();

    Ok(())
}
