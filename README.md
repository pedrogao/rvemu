# rvemu(RISC-V Emulator)

> risc-v emulator implement by Rust.

**Only support for Linux!**

## run

```sh
$ make fib.run
$ make csr.run
```

will output like this:

```sh
$ make fib.run
riscv64-unknown-elf-gcc -S scripts/fib.c -o scripts/fib.s
riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -march=rv64i -mabi=lp64 -o fib scripts/fib.s
/usr/local/lib/gcc/riscv64-unknown-elf/11.1.0/../../../../riscv64-unknown-elf/bin/ld: warning: cannot find entry symbol _start; defaulting to 0000000000000000
riscv64-unknown-elf-objcopy -O binary fib fib.bin
cargo run ./fib.bin
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/riscv-emulator ./fib.bin`

x00(zero)=0x80000030         x01( ra )=0x0                x02( sp )=0x88000000         x03( gp )=0x0
x04( tp )=0x0                x05( t0 )=0x0                x06( t1 )=0x0                x07( t2 )=0x0
x08( s0 )=0x0                x09( s1 )=0x0                x10( a0 )=0x37               x11( a1 )=0x0
x12( a2 )=0x0                x13( a3 )=0x0                x14( a4 )=0x1                x15( a5 )=0x37
x16( a6 )=0x0                x17( a7 )=0x0                x18( s2 )=0x0                x19( s3 )=0x0
x20( s4 )=0x0                x21( s5 )=0x0                x22( s6 )=0x0                x23( s7 )=0x0
x24( s8 )=0x0                x25( s9 )=0x0                x26( s10)=0x0                x27( s11)=0x0
x28( t3 )=0x0                x29( t4 )=0x0                x30( t5 )=0x0                x31( t6 )=0x0
-----------------------------------------------------------------------------------------------------------
mstatus=0x0                mtvec=0x0                mepc=0x0                mcause=0x0
sstatus=0x0                stvec=0x0                sepc=0x0                scause=0x0
```

## references

- [Writing a RISC-V Emulator in Rust](https://book.rvemu.app/index.html)
