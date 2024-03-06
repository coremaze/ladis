# Lazy Disassembler (ladis)

Lazy Disassembler (ladis) is a streamlined, command-line disassembler tool designed for converting hexadecimal machine code back into human-readable assembly language code. It supports a wide array of instruction sets, making it a versatile tool for developers, reverse engineers, and educational purposes where understanding the assembly code from binary data is crucial.

## Usage

Using Lazy Disassembler involves specifying the instruction set architecture (ARCH) of the input data and providing the hexadecimal data (HEX_DATA) you wish to disassemble. Here's how to use it:

```bash
ladis <ARCH> <HEX_DATA>
```

## Arguments

**\<ARCH\>**: The instruction set of the input data. Select from the supported architectures mentioned below.

**\<HEX_DATA\>**: The machine code data to disassemble, provided as a hexadecimal string.

## Options

`-h, --help`: Print help information.

## Example
To disassemble x86-32 hexadecimal machine code, you could use the following command:

```bash
$ ladis x86-32 "B8 05 00 00 00 83 C0 10 C3"
0000: B8 05 00 00 00                                  mov      eax, 5
0005: 83 C0 10                                        add      eax, 0x10
0008: C3                                              ret      
```

This command outputs the disassembled assembly code, showing the equivalent human-readable instructions.

## Features

- Supports a comprehensive range of instruction sets, accommodating a wide variety of architectures.
- Simple command-line interface for ease of use.
- Ideal for reverse engineering tasks, debugging, and educational purposes to help understand machine code.

## Supported Architectures

Lazy Disassembler supports disassembling machine code for the following instruction sets:

- ARM
- ARM-Thumb
- ARM64
- EVM
- Motorola 680x0 and 6811
- Motorola 68k040
- MIPS32
- MIPS64
- PPC32
- PPC64
- riscv32
- riscv64
- SPARC
- SystemZ
- TMS320C64x
- x86-16
- x86-32
- x86-64
- XCore
