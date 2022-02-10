pub mod cpu;
pub mod opcodes;
pub mod bus;

use cpu::Mem;
use cpu::CPU;
use bus::Bus;
use rand::Rng;
use std::time::Duration;
// SDLL
use spin_sleep;


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

fn main() {

    // load code for game
    let snake_code = vec![
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02,
        0x85, 0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9,
        0x0f, 0x85, 0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85,
        0x00, 0xa5, 0xfe, 0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20,
        0x8d, 0x06, 0x20, 0xc3, 0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c,
        0x38, 0x06, 0xa5, 0xff, 0xc9, 0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0,
        0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60, 0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85,
        0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0, 0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01,
        0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02, 0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05,
        0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06, 0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00,
        0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07, 0xe6, 0x03, 0xe6, 0x03, 0x20,
        0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06, 0xb5, 0x11, 0xc5, 0x11,
        0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c, 0x35, 0x07, 0x60,
        0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02, 0x4a, 0xb0,
        0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9, 0x20,
        0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10,
        0xb0, 0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5,
        0x10, 0x29, 0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe,
        0x91, 0x00, 0x60, 0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10,
        0x60, 0xa6, 0xff, 0xea, 0xea, 0xca, 0xd0, 0xfb, 0x60,
    ];

    pub struct OpCode {
        pub code:u8,
        pub mnemonic: &'static str,
        pub length: u8,
        pub cycles: u8,
        pub mode: AddressingModeMain,
    }
    
    impl OpCode {
        pub fn new(code: u8, mnemonic: &'static str, length: u8, cycles: u8, mode: AddressingModeMain) -> Self {
            OpCode {
                code : code,
                mnemonic : mnemonic,
                length : length,
                cycles : cycles,
                mode : mode,
            }
        }
    }

    pub enum AddressingModeMain {
        IMM, 
        ZP0, 
        ZPX,
        ZPY,
        ABS,
        ABX,
        ABY,
        IZX,
        IZY,
        NoneAddressing,
    }

    let codes: Vec<OpCode> = vec![
        // OpCode list taken from online resource
        OpCode::new(0x00, "BRK", 1, 7, AddressingModeMain::NoneAddressing),
        OpCode::new(0xea, "NOP", 1, 2, AddressingModeMain::NoneAddressing),

        /* Arithmetic */
        OpCode::new(0x69, "ADC", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0x65, "ADC", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x75, "ADC", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0x6d, "ADC", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0x7d, "ADC", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),
        OpCode::new(0x79, "ADC", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),
        OpCode::new(0x61, "ADC", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0x71, "ADC", 2, 5/*+1 if page crossed*/, AddressingModeMain::IZY),

        OpCode::new(0xe9, "SBC", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0xe5, "SBC", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0xf5, "SBC", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0xed, "SBC", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0xfd, "SBC", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),
        OpCode::new(0xf9, "SBC", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),
        OpCode::new(0xe1, "SBC", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0xf1, "SBC", 2, 5/*+1 if page crossed*/, AddressingModeMain::IZY),

        OpCode::new(0x29, "AND", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0x25, "AND", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x35, "AND", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0x2d, "AND", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0x3d, "AND", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),
        OpCode::new(0x39, "AND", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),
        OpCode::new(0x21, "AND", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0x31, "AND", 2, 5/*+1 if page crossed*/, AddressingModeMain::IZY),

        OpCode::new(0x49, "EOR", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0x45, "EOR", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x55, "EOR", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0x4d, "EOR", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0x5d, "EOR", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),
        OpCode::new(0x59, "EOR", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),
        OpCode::new(0x41, "EOR", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0x51, "EOR", 2, 5/*+1 if page crossed*/, AddressingModeMain::IZY),

        OpCode::new(0x09, "ORA", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0x05, "ORA", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x15, "ORA", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0x0d, "ORA", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0x1d, "ORA", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),
        OpCode::new(0x19, "ORA", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),
        OpCode::new(0x01, "ORA", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0x11, "ORA", 2, 5/*+1 if page crossed*/, AddressingModeMain::IZY),

        /* Shifts */
        OpCode::new(0x0a, "ASL", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x06, "ASL", 2, 5, AddressingModeMain::ZP0),
        OpCode::new(0x16, "ASL", 2, 6, AddressingModeMain::ZPX),
        OpCode::new(0x0e, "ASL", 3, 6, AddressingModeMain::ABS),
        OpCode::new(0x1e, "ASL", 3, 7, AddressingModeMain::ABX),

        OpCode::new(0x4a, "LSR", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x46, "LSR", 2, 5, AddressingModeMain::ZP0),
        OpCode::new(0x56, "LSR", 2, 6, AddressingModeMain::ZPX),
        OpCode::new(0x4e, "LSR", 3, 6, AddressingModeMain::ABS),
        OpCode::new(0x5e, "LSR", 3, 7, AddressingModeMain::ABX),

        OpCode::new(0x2a, "ROL", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x26, "ROL", 2, 5, AddressingModeMain::ZP0),
        OpCode::new(0x36, "ROL", 2, 6, AddressingModeMain::ZPX),
        OpCode::new(0x2e, "ROL", 3, 6, AddressingModeMain::ABS),
        OpCode::new(0x3e, "ROL", 3, 7, AddressingModeMain::ABX),

        OpCode::new(0x6a, "ROR", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x66, "ROR", 2, 5, AddressingModeMain::ZP0),
        OpCode::new(0x76, "ROR", 2, 6, AddressingModeMain::ZPX),
        OpCode::new(0x6e, "ROR", 3, 6, AddressingModeMain::ABS),
        OpCode::new(0x7e, "ROR", 3, 7, AddressingModeMain::ABX),

        OpCode::new(0xe6, "INC", 2, 5, AddressingModeMain::ZP0),
        OpCode::new(0xf6, "INC", 2, 6, AddressingModeMain::ZPX),
        OpCode::new(0xee, "INC", 3, 6, AddressingModeMain::ABS),
        OpCode::new(0xfe, "INC", 3, 7, AddressingModeMain::ABX),

        OpCode::new(0xe8, "INX", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0xc8, "INY", 1, 2, AddressingModeMain::NoneAddressing),

        OpCode::new(0xc6, "DEC", 2, 5, AddressingModeMain::ZP0),
        OpCode::new(0xd6, "DEC", 2, 6, AddressingModeMain::ZPX),
        OpCode::new(0xce, "DEC", 3, 6, AddressingModeMain::ABS),
        OpCode::new(0xde, "DEC", 3, 7, AddressingModeMain::ABX),

        OpCode::new(0xca, "DEX", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x88, "DEY", 1, 2, AddressingModeMain::NoneAddressing),

        OpCode::new(0xc9, "CMP", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0xc5, "CMP", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0xd5, "CMP", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0xcd, "CMP", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0xdd, "CMP", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),
        OpCode::new(0xd9, "CMP", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),
        OpCode::new(0xc1, "CMP", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0xd1, "CMP", 2, 5/*+1 if page crossed*/, AddressingModeMain::IZY),

        OpCode::new(0xc0, "CPY", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0xc4, "CPY", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0xcc, "CPY", 3, 4, AddressingModeMain::ABS),

        OpCode::new(0xe0, "CPX", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0xe4, "CPX", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0xec, "CPX", 3, 4, AddressingModeMain::ABS),


        /* Branching */

        OpCode::new(0x4c, "JMP", 3, 3, AddressingModeMain::NoneAddressing), //AddressingModeMain that acts as Immidiate
        OpCode::new(0x6c, "JMP", 3, 5, AddressingModeMain::NoneAddressing), //AddressingModeMain:Indirect with 6502 bug

        OpCode::new(0x20, "JSR", 3, 6, AddressingModeMain::NoneAddressing),
        OpCode::new(0x60, "RTS", 1, 6, AddressingModeMain::NoneAddressing),

        OpCode::new(0x40, "RTI", 1, 6, AddressingModeMain::NoneAddressing),

        OpCode::new(0xd0, "BNE", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),
        OpCode::new(0x70, "BVS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),
        OpCode::new(0x50, "BVC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),
        OpCode::new(0x30, "BMI", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),
        OpCode::new(0xf0, "BEQ", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),
        OpCode::new(0xb0, "BCS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),
        OpCode::new(0x90, "BCC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),
        OpCode::new(0x10, "BPL", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingModeMain::NoneAddressing),

        OpCode::new(0x24, "BIT", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x2c, "BIT", 3, 4, AddressingModeMain::ABS),


        /* Stores, Loads */
        OpCode::new(0xa9, "LDA", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0xad, "LDA", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0xbd, "LDA", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),
        OpCode::new(0xb9, "LDA", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0xb1, "LDA", 2, 5/*+1 if page crossed*/, AddressingModeMain::IZY),

        OpCode::new(0xa2, "LDX", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0xa6, "LDX", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0xb6, "LDX", 2, 4, AddressingModeMain::ZPY),
        OpCode::new(0xae, "LDX", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0xbe, "LDX", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABY),

        OpCode::new(0xa0, "LDY", 2, 2, AddressingModeMain::IMM),
        OpCode::new(0xa4, "LDY", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0xb4, "LDY", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0xac, "LDY", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0xbc, "LDY", 3, 4/*+1 if page crossed*/, AddressingModeMain::ABX),


        OpCode::new(0x85, "STA", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x95, "STA", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0x8d, "STA", 3, 4, AddressingModeMain::ABS),
        OpCode::new(0x9d, "STA", 3, 5, AddressingModeMain::ABX),
        OpCode::new(0x99, "STA", 3, 5, AddressingModeMain::ABY),
        OpCode::new(0x81, "STA", 2, 6, AddressingModeMain::IZX),
        OpCode::new(0x91, "STA", 2, 6, AddressingModeMain::IZY),

        OpCode::new(0x86, "STX", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x96, "STX", 2, 4, AddressingModeMain::ZPY),
        OpCode::new(0x8e, "STX", 3, 4, AddressingModeMain::ABS),

        OpCode::new(0x84, "STY", 2, 3, AddressingModeMain::ZP0),
        OpCode::new(0x94, "STY", 2, 4, AddressingModeMain::ZPX),
        OpCode::new(0x8c, "STY", 3, 4, AddressingModeMain::ABS),


        /* Flags clear */

        OpCode::new(0xD8, "CLD", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x58, "CLI", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0xb8, "CLV", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x18, "CLC", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x38, "SEC", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x78, "SEI", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0xf8, "SED", 1, 2, AddressingModeMain::NoneAddressing),

        OpCode::new(0xaa, "TAX", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0xa8, "TAY", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0xba, "TSX", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x8a, "TXA", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x9a, "TXS", 1, 2, AddressingModeMain::NoneAddressing),
        OpCode::new(0x98, "TYA", 1, 2, AddressingModeMain::NoneAddressing),

        /* Stack */
        OpCode::new(0x48, "PHA", 1, 3, AddressingModeMain::NoneAddressing),
        OpCode::new(0x68, "PLA", 1, 4, AddressingModeMain::NoneAddressing),
        OpCode::new(0x08, "PHP", 1, 3, AddressingModeMain::NoneAddressing),
        OpCode::new(0x28, "PLP", 1, 4, AddressingModeMain::NoneAddressing),
    ];

    // load the game
    /*
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.load(snake_code);
    cpu.reset();
    cpu.program_counter = 0x0600;

    let mut screen_state = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();
    // game cycle
    */
    let command: &str = "LDA";

    for item in &codes {
        if command == item.mnemonic {
             println!("Code found = {:X}", item.code);
         }
    }
}
