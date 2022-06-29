mod bus;
mod cpu;
mod dram;

use std::{
    env,
    fs::File,
    io::{self, Read},
};

use cpu::*;
use dram::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: riscv-emulator <filename>")
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    loop {
        // 1. Fetch instruction
        let inst = match cpu.fetch() {
            // Break the loop if an error occurs.
            Ok(inst) => inst,
            Err(_) => break,
        };

        // 2. Add pc 4, instruction's length is 32 bits
        cpu.pc += 4;

        // 3. Decode & Execute
        match cpu.execute(inst) {
            Ok(_) => {}
            Err(_) => break,
        }

        if cpu.pc == 0 {
            break;
        }
    }
    cpu.dump_registers();

    Ok(())
}
