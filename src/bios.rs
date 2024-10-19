use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use crate::cpu::CPU;
use crate::ram::RAM_SIZE;
use crate::utils::parse_address;
use crate::assembler::Assembler;

pub struct BIOS {
    pub cpu: CPU,
    assembler: Assembler,
}

impl BIOS {
    pub fn new(cpu: CPU) -> Self {
        BIOS { 
            cpu,
            assembler: Assembler::new(),
        }
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
                self.handle_address_command(command);
            } else if command.starts_with("memory_dump") {
                self.handle_memory_dump_command();
            } else if command.ends_with(".asm") {
                self.handle_asm_file(command);
            } else if command.starts_with("compile") {
                self.handle_compile_command(command);
            } else if command.starts_with("run") {
                self.handle_run_command(command);
            } else {
                println!("Unknown command: {}", command);
            }
        }
    }
    
    fn handle_compile_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Usage: compile <input_file.asm> <output_file.bin>");
            return;
        }
        match self.assembler.assemble(parts[1], parts[2]) {
            Ok(_) => println!("Successfully compiled {} to {}", parts[1], parts[2]),
            Err(e) => println!("Error compiling file: {}", e),
        }
    }

    fn handle_address_command(&self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter the address you'd like to gather!");
            return;
        }
        match parse_address(parts[1]) {
            Ok(address) => match self.cpu.ram.read(address) {
                Ok(value) => println!("Value at address {}: {:08b}", parts[1], value),
                Err(e) => println!("Error: {}", e),
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    fn handle_memory_dump_command(&self) {
        let dump = self.cpu.ram.dump(0, RAM_SIZE);
        println!("Memory Dump:\n{}", dump.join("\n"));
    }

    fn handle_asm_file(&mut self, filename: &str) {
        if let Err(e) = self.execute_asm(filename) {
            println!("Error running program: {}", e);
        }
    }

    fn handle_run_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Usage: run <binary_file.bin>");
            return;
        }
        match self.execute_binary(parts[1]) {
            Ok(_) => println!("Program execution completed"),
            Err(e) => println!("Error running binary: {}", e),
        }
    }

    pub fn execute_asm(&mut self, filename: &str) -> io::Result<()> {
        let lines = read_lines(filename)?;
        println!("Running program: {}", filename);
        for line in lines {
            let instruction = line?;
            match self.cpu.execute(&instruction) {
                Ok(continue_execution) => {
                    if !continue_execution {
                        println!("Program halted.");
                        break;
                    }
                }
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::Other, e));
                }
            }
        }
        Ok(())
    }

    pub fn execute_binary(&mut self, filename: &str) -> io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Load binary into RAM
        for (i, &byte) in buffer.iter().enumerate() {
            self.cpu.ram.write(i, byte).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        // Execute from the beginning of RAM
        let mut pc = 0;
        loop {
            let opcode = self.cpu.ram.read(pc).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            match opcode {
                0x01 => { // LOAD
                    let reg = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let addr = self.cpu.ram.read(pc + 2).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let value = self.cpu.ram.read(addr).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    self.cpu.registers[reg] = value;
                }
                0x02 => { // STORE
                    let reg = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let addr = self.cpu.ram.read(pc + 2).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let value = self.cpu.registers[reg];
                    self.cpu.ram.write(addr, value).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                }
                0x03 => { // ADD
                    let r1 = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r2 = self.cpu.ram.read(pc + 2).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r3 = self.cpu.ram.read(pc + 3).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    self.cpu.registers[r3] = self.cpu.registers[r1].wrapping_add(self.cpu.registers[r2]);
                }
                0x04 => { // AND
                    let r1 = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r2 = self.cpu.ram.read(pc + 2).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r3 = self.cpu.ram.read(pc + 3).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    self.cpu.registers[r3] = self.cpu.registers[r1] & self.cpu.registers[r2];
                }
                0x05 => { // OR
                    let r1 = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r2 = self.cpu.ram.read(pc + 2).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r3 = self.cpu.ram.read(pc + 3).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    self.cpu.registers[r3] = self.cpu.registers[r1] | self.cpu.registers[r2];
                }
                0x06 => { // XOR
                    let r1 = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r2 = self.cpu.ram.read(pc + 2).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r3 = self.cpu.ram.read(pc + 3).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    self.cpu.registers[r3] = self.cpu.registers[r1] ^ self.cpu.registers[r2];
                }
                0x07 => { // NOT
                    let r1 = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    let r2 = self.cpu.ram.read(pc + 2).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? as usize;
                    self.cpu.registers[r2] = !self.cpu.registers[r1];
                }
                0x08 => { // JUMP
                    let addr = self.cpu.ram.read(pc + 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    pc = addr as usize;
                    continue; // Skip the pc increment at the end of the loop
                }
                0xFF => { // HALT
                    break;
                }
                _ => {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("Invalid opcode: {:02X}", opcode)));
                }
            }

            pc += 4; // Advance to the next instruction (each instruction is 4 bytes)
        }

        Ok(())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
