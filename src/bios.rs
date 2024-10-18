use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::cpu::CPU;
use crate::ram::RAM_SIZE;
use crate::utils::parse_address;

pub struct BIOS {
    pub cpu: CPU,
}

impl BIOS {
    pub fn new(cpu: CPU) -> Self {
        BIOS { cpu }
    }

    pub fn prompt(&mut self) {
        loop {
            print!("BIOS> ");
            io::stdout().flush().unwrap();

            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            let command = command.trim();

            if command == "exit" {
                break;
            } else if command.starts_with("address") {
                let parts: Vec<&str> = command.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("Please enter the address you'd like to gather!");
                    continue;
                }
                match parse_address(parts[1]) {
                    Ok(address) => match self.cpu.ram.read(address) {
                        Ok(value) => println!("Value at address {}: {:08b}", parts[1], value),
                        Err(e) => println!("Error: {}", e),
                    },
                    Err(e) => println!("Error: {}", e),
                }
            } else if command.starts_with("memory_dump") {
                let dump = self.cpu.ram.dump(0, RAM_SIZE);
                println!("Memory Dump:\n{}", dump.join("\n"));
            } else if command.ends_with(".asm") {
                self.run_program(command);
            } else {
                println!("Unknown command: {}", command);
            }
        }
    }

    fn run_program(&mut self, filename: &str) {
        match read_lines(filename) {
            Ok(lines) => {
                let program: Vec<String> = lines.filter_map(Result::ok).collect();
                println!("Running program: {}", filename);
                for instruction in program {
                    match self.cpu.execute(&instruction) {
                        Ok(continue_execution) => {
                            if !continue_execution {
                                println!("Program halted.");
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Error executing instruction '{}': {}", instruction, e);
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("Error reading file '{}': {}", filename, e),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
