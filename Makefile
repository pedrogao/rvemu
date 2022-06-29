add.bin:
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o add scripts/add.s
	riscv64-unknown-elf-objcopy -O binary add add.bin

add.run: add.bin
	cargo run ./add.bin

fib.bin:
	riscv64-unknown-elf-gcc -S scripts/fib.c -o scripts/fib.s
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -march=rv64i -mabi=lp64 -o fib scripts/fib.s
	riscv64-unknown-elf-objcopy -O binary fib fib.bin

fib.run: fib.bin
	cargo run ./fib.bin

csr.bin:
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o csr scripts/csr.s
	riscv64-unknown-elf-objcopy -O binary csr csr.bin

csr.run: csr.bin
	cargo run ./csr.bin

clean:
	rm add || true
	rm fib || true
	rm csr || true
	rm *.bin || true
