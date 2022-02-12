pub mod cpu;
pub mod opcodes;
pub mod bus;

use cpu::CPU;
use bus::Bus;
use cpu::Flags;
use std::fs;
use asm6502::assemble;
use std::env;


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

fn pretty_print_flags(cpu: &mut CPU) {
    let mut pretty_flags: Vec<String> = vec![];
    if cpu.status.contains(Flags::NEGATIVE) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }

    if cpu.status.contains(Flags::OVERFLOW) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }

    if cpu.status.contains(Flags::BREAK2) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }

    if cpu.status.contains(Flags::BREAK) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }
    
    if cpu.status.contains(Flags::DECIMAL) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }

    if cpu.status.contains(Flags::INTERRUPT) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }

    if cpu.status.contains(Flags::ZERO) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }

    if cpu.status.contains(Flags::CARRY) {
        pretty_flags.push(String::from("1"));
    } else {
        pretty_flags.push(String::from("0"));
    }

    let (first, rest) = pretty_flags.split_first_mut().unwrap();
    first.push_str(" ");
    for item in rest.iter() {
        first.push_str(item.as_str());
        first.push_str(" ");
    }
    println!("{}", &first);
}

fn print_regs(cpu: &mut CPU) {
    println!("********************");
    println!("Flags Status:       ");
    println!("N O B B D I Z C     ");
    pretty_print_flags(cpu);
    println!(                      );
    println!("Registers:          ");
    println!("A: {:#02x}  X: {:#02x}  Y: {:#02x}", cpu.reg_a, cpu.reg_x, cpu.reg_y);
    println!("********************");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];

    //let filename = "src/test.asm";
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    println!("With text:\n{}", contents);

    let commands = contents.as_bytes();
    //let commands = "LDA #$01\nSTA $0200\nLDA #$05\nSTA $0201\nLDA #$08\nSTA $0202".as_bytes();
    let mut buf = Vec::<u8>::new();
    if let Err(msg) = assemble(commands, &mut buf) {
        panic!("Failed to assemble: {}", msg);
    }

    println!("Machine code assembled:");
    println!("{:?}", buf);

    // load the game
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.load(buf);
    cpu.reset();
    cpu.program_counter = 0x0600;

    use std::io::{stdin, stdout, Write};
    use std::process;
    let mut cont_flag = 0;
    let mut break_flag = 0;
    let mut s = String::new();
    println!("Please enter a character to continue (c to continue, s to step, z to exit): \n");
    // game cycle
    cpu.run_with_callback(move |cpu| {
        break_flag = 0;
        print_regs(cpu);
        if cont_flag == 0 {
            while break_flag == 0 {
                let _=stdout().flush();
                stdin().read_line(&mut s).expect("Did not enter a correct string");
                let trimmed = s.trim();
                match trimmed {
                    "c" => {
                        println!("continue received");
                        cont_flag = 1; 
                        break_flag = 1;
                    }
                    "s" => {
                        println!("step received");
                        break_flag = 1;
                    }
                    "z" => {
                        println!("done received");
                        process::exit(0);
                    }
                    _ => {
                        println!("Invalid input...");
                        println!("Please enter a character to continue (c to continue, s to step, z to exit): \n");
                    }
                }
                s.clear();
            }
        }
    });
}
