# rvcore

A RISC-V RV64I emulator written in Rust. The goal of the project is to understand the architecture at the instruction level and to build a core that future projects can be layered on top of. The emulator is a pure software interpretation of instructions through a `fetch-decode-execute` loop, with no hardware virtualization.

## Running

All commands are run from the `src` folder.

Compiling a test program into an executable binary:

```
make ../test/<program_name>.bin
```

Running the emulator on that program:

```
cargo run -- ../test/<program_name>.bin
```

Test programs are compiled without compressed instructions (`-march=rv64i`, or `-march=rv64im` for the ones that use `div`/`rem`), since the emulator reads fixed-width 4-byte instructions.

## What is RISC-V

RISC-V is an open ISA (instruction set architecture) based on RISC principles: a small number of simple, regular, fixed-length instructions, with the entire standard being fully open. It is modular: there is a base integer set (I), onto which optional extensions are added (M for multiplication and division, A for atomic operations, and others). This project implements RV64I, the 64-bit base integer set.

## Implementation details

### Project structure

The files are arranged to reflect how a real computer works, only in a miniature version. Each module corresponds to one component of the system, and the layers communicate just as they do in hardware: the processor talks to memory over the bus, not directly.

`cpu.rs` is the processor. It holds 32 general-purpose registers and the program counter (`pc`), and contains the `fetch-decode-execute` loop: it fetches the instruction at the address `pc` points to, decodes it (extracts the fields), then executes it.

`bus.rs` is the system bus. It routes every memory access by address: if the address falls within a device's range, it forwards the request to that device, otherwise it goes to RAM. The processor never accesses memory directly, always through the bus, just like in a real system.

`dram.rs` is the main memory. It holds bytes and serves reads and writes of various widths (8, 16, 32, 64 bits).

`uart.rs` is the serial output (teleprinter). It is a memory-mapped device: when the processor writes a byte to its address, that byte is printed to the screen.

This layered design was chosen deliberately so that new devices (timer, interrupt controller, disk) and new capabilities (virtual memory) can be added later without touching the existing components.

### Von Neumann architecture

The emulator follows the von Neumann model: code and data share the same memory space. Instructions and data live in the same RAM, and the processor distinguishes them only by how it accesses them (`fetch` for instructions, `load`/`store` for data). This is why a program and its data are loaded into the same memory starting from the base address `0x8000_0000`.

### Little-endian

Memory is organized little-endian: for multi-byte values, the least significant byte is stored at the lowest address. The value `0x12345678` is laid out in memory as the byte sequence `78 56 34 12`. Every read and write of 16/32/64-bit values assembles or disassembles the bytes in that order.

### The Result type for error handling

Every memory operation and every instruction returns Rust's generic `Result<T, E>` type, regardless of whether it can fail (access to an invalid address, an unknown instruction). `Result` has two variants: `Ok(T)` carries a successful result, `Err(E)` signals an error. This way errors propagate through the layers (DRAM → bus → processor) explicitly, instead of being ignored or causing the program to crash. The `?` operator automatically forwards an error upward if one occurs, keeping the code clean and the error handling consistent.

## Instruction types

RISC-V has several instruction formats (I, R, S, B, U, J), and the key rule that ties them together is that **rd, rs1, rs2, funct3, and funct7 always sit at the same positions** across all formats. The reason is hardware simplicity: the processor always pulls the registers from the same bits without caring about the instruction format, and the decoder does not have to shift fields around.

A consequence of that rule is that the immediate (constant) only gets the bits that remain once the registers and the opcode have taken their fixed positions, so in some formats it ends up scattered.

![RISC-V instruction types](assets/rv_instr_types.png)

The most striking example of this scattering is the B-type (branches). Its immediate is deliberately arranged to **resemble the S-type** (store) as closely as possible, since B and S are nearly identical (both have two registers and a constant). This lets the hardware share the immediate-extraction logic between the two formats. The price of that overlap is that one bit (bit 11) has to "jump" to an unusual place, because its natural position is taken in the S-type. Also, the highest bit (the sign) is at bit 31 of the instruction in every format, so that sign-extension is simple and shared across all formats.

## Implemented instruction set

The emulator covers the entire base RV64I set:

- Arithmetic and logic, register-register (`add`, `sub`, `and`, `or`, `xor`, `slt`, `sltu`) and register-immediate (`addi`, `andi`, `ori`, `xori`, `slti`, `sltiu`)
- Shifts (`sll`, `srl`, `sra` and immediate variants), with a 6-bit shift amount for RV64
- Loads and stores of all widths (`lb`, `lh`, `lw`, `ld` with signed/unsigned variants, `sb`, `sh`, `sw`, `sd`)
- Working with large constants (`lui`, `auipc`)
- Branches (`beq`, `bne`, `blt`, `bge`, `bltu`, `bgeu`)
- Jumps (`jal`, `jalr`)
- 32-bit W-variants (`addw`, `subw`, `sllw`, `srlw`, `sraw` and immediate variants), which operate on the lower 32 bits and then sign-extend the result to 64

Additionally, beyond the pure I set and as preparation for further development, `div` and `rem` from the M extension and the `UART` output are implemented.

### Implementation highlights

**`sltiu rd, rs1, 1` as a zero check.** The `sltiu` instruction compares unsigned and sets the result to 1 if rs1 is less than the constant. Since the only unsigned number less than 1 is zero itself, `sltiu rd, rs1, 1` yields 1 exactly when rs1 equals zero. RISC-V has no direct "equals zero" instruction, so compilers use precisely this idiom. The subtlety is that the immediate in `sltiu` is still sign-extended (as in all I-type instructions), and only the comparison is unsigned.

**Building large constants with `lui` + `addi`.** Since an ordinary instruction only has room for a 12-bit constant, larger values are built in two steps: `lui` places the upper 20 bits into a register, then `addi` adds the lower 12. For example, to bring the UART address `0x10000000` into a register, a single `lui` with `0x10000` is enough (since the lower 12 bits are zero). This pattern is the basis for addressing any memory location whose address does not fit in 12 bits.

## Test programs

The `test` folder contains assembly programs, each exercising one group of instructions and producing a predictable result in the registers:

- `fibonacci.s` computes F(10) iteratively (result 55), exercises branching and a loop
- `multiply_loop.s` multiplies through repeated addition (5×4 = 20), since the pure I set has no multiplication
- `alu_test.s` exercises arithmetic, logic, and shifts with verifiable results
- `loadstore_test.s` writes then reads from memory, a full cycle through DRAM
- `jumps_test.s` demonstrates a call to and return from a "function" via `jal`/`jalr`

## UART and Hello World

A minimal UART (model 16550) is implemented, memory-mapped at `0x10000000`, working as a primitive teleprinter: when the program writes a byte to its address, the byte is printed to standard output. The status register always returns "ready to transmit", so the program does not have to wait.

Alongside the UART, a `hello_world` assembly program writes the ASCII values of the characters one by one to the UART address and prints "Hello World!" to the screen. This is the first moment when the emulated program communicates with the outside world on its own, instead of the result being read out of the registers.

## Further development

Now that there is a stable core, the plan is to keep building on top of it:

- **A hardware accelerator** as a memory-mapped device, for experimenting with speeding up specific operations
- **Booting xv6**, which requires adding the M and A extensions, CSR registers, privileged modes, the trap mechanism, interrupt controllers (CLINT/PLIC), and Sv39 virtual memory, plus a more advanced monitor
- **A JIT compiler** that translates RISC-V instructions into native code at runtime, instead of interpreting instruction by instruction, for performance