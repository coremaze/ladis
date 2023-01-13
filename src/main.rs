use capstone::{prelude::*, Capstone, Error};
use clap::{Parser, ValueEnum};
use hex::FromHex;

#[derive(Debug, ValueEnum, Clone)]
enum Architecture {
    Arm,
    ArmThumb,
    Arm64,
    Evm,
    M680x6811,
    M68k040,
    Mips32,
    Mips64,
    Ppc32,
    Ppc64,
    Riscv32,
    Riscv64,
    Sparc,
    Sysz,
    Tms320c64x,
    X86_32,
    X86_64,
    Xcore,
}

#[derive(Parser)]
struct Args {
    /// The instruction set of the input data
    pub arch: Architecture,

    /// The data to disassemble, as hexadecimal
    pub hex_data: String,
}

fn get_capstone(arch: Architecture) -> Result<Capstone, Error> {
    match arch {
        Architecture::Arm => Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Arm)
            .detail(true)
            .build(),
        Architecture::ArmThumb => Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Thumb)
            .detail(true)
            .build(),
        Architecture::Arm64 => Capstone::new()
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .detail(true)
            .build(),
        Architecture::Evm => Capstone::new()
            .evm()
            .mode(arch::evm::ArchMode::Default)
            .detail(true)
            .build(),
        Architecture::M680x6811 => Capstone::new()
            .m680x()
            .mode(arch::m680x::ArchMode::M680x6811)
            .detail(true)
            .build(),
        Architecture::M68k040 => Capstone::new()
            .m68k()
            .mode(arch::m68k::ArchMode::M68k040)
            .detail(true)
            .build(),
        Architecture::Mips32 => Capstone::new()
            .mips()
            .mode(arch::mips::ArchMode::Mips32)
            .detail(true)
            .build(),
        Architecture::Mips64 => Capstone::new()
            .mips()
            .mode(arch::mips::ArchMode::Mips64)
            .detail(true)
            .build(),
        Architecture::Ppc32 => Capstone::new()
            .ppc()
            .mode(arch::ppc::ArchMode::Mode32)
            .detail(true)
            .build(),
        Architecture::Ppc64 => Capstone::new()
            .ppc()
            .mode(arch::ppc::ArchMode::Mode64)
            .detail(true)
            .build(),
        Architecture::Riscv32 => Capstone::new()
            .riscv()
            .mode(arch::riscv::ArchMode::RiscV32)
            .detail(true)
            .build(),
        Architecture::Riscv64 => Capstone::new()
            .riscv()
            .mode(arch::riscv::ArchMode::RiscV64)
            .detail(true)
            .build(),
        Architecture::Sparc => Capstone::new()
            .sparc()
            .mode(arch::sparc::ArchMode::Default)
            .detail(true)
            .build(),
        Architecture::Sysz => Capstone::new()
            .sysz()
            .mode(arch::sysz::ArchMode::Default)
            .detail(true)
            .build(),
        Architecture::Tms320c64x => Capstone::new()
            .tms320c64x()
            .mode(arch::tms320c64x::ArchMode::Default)
            .detail(true)
            .build(),
        Architecture::X86_32 => Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode32)
            .syntax(arch::x86::ArchSyntax::Intel)
            .detail(true)
            .build(),
        Architecture::X86_64 => Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode64)
            .syntax(arch::x86::ArchSyntax::Intel)
            .detail(true)
            .build(),
        Architecture::Xcore => Capstone::new()
            .xcore()
            .mode(arch::xcore::ArchMode::Default)
            .detail(true)
            .build(),
    }
}

fn main() {
    let args = Args::parse();

    let cs = match get_capstone(args.arch) {
        Ok(cs) => cs,
        Err(why) => {
            eprintln!("Failed to build disassembler: {why}");
            return;
        }
    };

    let hex_data: String = args
        .hex_data
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect();

    let bytes = match <Vec<u8>>::from_hex(hex_data) {
        Ok(data) => data,
        Err(why) => {
            eprintln!("Hex data input could not be parsed: {why}");
            return;
        }
    };

    let instructions = match cs.disasm_all(&bytes, 0) {
        Ok(ins) => ins,
        Err(why) => {
            eprintln!("Could not disassemble bytes: {why}");
            return;
        }
    };

    println!("{instructions}");
}
