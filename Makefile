all:
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o add scripts/add.s
	riscv64-unknown-elf-objcopy -O binary add add.bin
run:
	cargo run ./add.bin

clean:
	rm add
	rm add.bin
