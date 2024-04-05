# RISC-V Sim

This is a RISC-V simulator written in rust. I decided to do this project mainly for educational purposes.
Everybody is welcome to use it as they want to, submit contributions or file in bug reports or feature requests :)

## What is RISC-V?

From [Wikipedia](https://en.wikipedia.org/wiki/RISC-V): RISC-V is an open standard intruction set architecture base on established reduced instruction set computer (RISC) principles.

### Instruction set

Also form Wikipedia: RISC-V has a modular design, consisting of alternative base parts, with added optional extensions. The base specifies instructions, control flow, memory and adressing, locig manipulation and ancilliaries.

For simplicity, the SIM we'll start implementing the [RV32I](https://five-embeddev.com/riscv-isa-manual/latest/rv32i.html).

### Registers
The base interger ISA, RV32I has 32 registers, which are 32 bit wide each. x0 is hardwired with all bits equal to 0, and the registers x1 to x31 are all general purpose registers. There is one extra register which is the program counter(PC).

There is no dedicated stack pointer or subroutine addres link register in the Base Integer ISA. However the standard software convetion uses x1 to hold the return address for a call, x5 is an alternate link register and x2 is used as the stack pointer.

### TODO
#### Near term
* Implement remaining instruction functions.
* Load a binary file, decode it and execute it.
* Command line user interface: load file, run, step, print registers.
#### Long term
* Graphical user interface. Maybe in browser.
* Implement more architecture extensions. Multiply, atomic, floating point, 64-bit.
* Debugging: add breakpoints, step back functionality.
