use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::utils::{parse_address, parse_immediate, parse_register};

pub struct Assembler {
    instruction_set: HashMap<String, u8>,
    labels: HashMap<String, usize>,
}

impl Assembler {
    pub fn new() -> Self {
        let mut instruction_set = HashMap::new();

        instruction_set.insert("LOAD".to_string(), 0x01);
        instruction_set.insert("STORE".to_string(), 0x02);
        instruction_set.insert("ADD".to_string(), 0x03);
        instruction_set.insert("AND".to_string(), 0x04);
        instruction_set.insert("OR".to_string(), 0x05);
        instruction_set.insert("XOR".to_string(), 0x06);
        instruction_set.insert("NOT".to_string(), 0x07);
        instruction_set.insert("JUMP".to_string(), 0x08);
        instruction_set.insert("INIT".to_string(), 0x09);
        instruction_set.insert("OUT".to_string(), 0xA);
        instruction_set.insert("CLEAR".to_string(), 0xB);
        instruction_set.insert("VER".to_string(), 0xFE);
        instruction_set.insert("HALT".to_string(), 0xFF);

        Assembler {
            instruction_set,
            labels: HashMap::new(),
        }
    }

    pub fn assemble(&mut self, input_file: &str, output_file: &str) -> io::Result<()> {
        let lines = self.read_lines(input_file)?;
        let mut binary = Vec::new();

        // First pass: collect labels
        for (line_number, line) in lines.iter().enumerate() {
            let line = line.split(';').next().unwrap().trim();
            if let Some(label) = line.strip_suffix(':') {
                self.labels.insert(label.to_string(), line_number);
            }
        }

        // Second pass: convert instructions to binary
        for line in lines {
            let line = line.split(';').next().unwrap().trim();
            if line.ends_with(':') || line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            let opcode = self.instruction_set.get(parts[0]).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Unknown instruction: {}", parts[0]))
            })?;
            binary.push(*opcode);

            if parts.len() > 1 {
                println!("{:?}", parts);
                match parts[0] {
                    "INIT" => {
                        // Correctly split and parse INIT with address and immediate value
                        let address = parse_address(parts[1]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid address: {}", e))
                        })?;
                        binary.push(address as u8);

                        let value = parse_immediate(parts[3]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid value: {}", e))
                        })?;
                        binary.push(value);
                    }
                    "VER" => {
                        let value = parse_immediate(parts[parts.len()-1]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid value: {}", e))
                        })?;
                        binary.push(value);
                    }
                    "LOAD" | "STORE" => {
                        let reg = parse_register(parts[1]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid register: {}", e))
                        })?;
                        binary.push(reg as u8);

                        let address = parse_address(parts[2]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid address: {}", e))
                        })?;
                        binary.push(address as u8);
                    }
                    "ADD" => {
                        let reg1 = parse_register(parts[1]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid register: {}", e))
                        })?;
                        binary.push(reg1 as u8);

                        let reg2 = parse_register(parts[2]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid register: {}", e))
                        })?;
                        binary.push(reg2 as u8);

                        let reg3 = parse_register(parts[3]).map_err(|e| {
                            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid register: {}", e))
                        })?;
                        binary.push(reg3 as u8);
                    }
                    _ => {
                        for operand in &parts[1..] {
                            if let Some(label_address) = self.labels.get(*operand) {
                                binary.push(*label_address as u8);
                            } else if operand.starts_with('R') {
                                let reg = parse_register(operand).map_err(|e| {
                                    io::Error::new(io::ErrorKind::InvalidData, format!("Invalid register: {}", e))
                                })?;
                                binary.push(reg as u8);
                            } else if operand.starts_with('[') && operand.ends_with(']') {
                                let address = parse_address(operand).map_err(|e| {
                                    io::Error::new(io::ErrorKind::InvalidData, format!("Invalid address: {}", e))
                                })?;
                                binary.push(address as u8);
                            } else {
                                let value = parse_immediate(operand).map_err(|e| {
                                    io::Error::new(io::ErrorKind::InvalidData, format!("Invalid value: {}", e))
                                })?;
                                binary.push(value);
                            }
                        }
                    }
                }
            }
        }

        // Write binary to output file
        let mut file = File::create(output_file)?;
        file.write_all(&binary)?;
        Ok(())
    }

    fn read_lines<P>(&self, filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        io::BufReader::new(file).lines().collect()
    }
}
