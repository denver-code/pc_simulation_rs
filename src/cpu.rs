use crate::ram::RAM;
use crate::logic_gates::LogicGates;

pub struct CPU {
    pub ram: RAM,
    registers: [u8; 8],
    verbose: bool,
}

impl CPU {
    pub fn new(ram: RAM) -> Self {
        CPU {
            ram,
            registers: [0; 8],
            verbose: false,
        }
    }
    
    fn parse_register(&self, reg: &str) -> Result<usize, String> {
        let reg = reg.trim_end_matches(',');
        if !reg.starts_with('R') {
            return Err(format!("Invalid register format: {}", reg));
        }
        reg[1..].parse::<usize>()
            .map_err(|e| format!("Failed to parse register number: {}", e))
    }

    fn parse_address(&self, addr: &str) -> Result<usize, String> {
        let addr = addr.trim_end_matches(',');
        if !addr.starts_with('[') || !addr.ends_with(']') {
            return Err(format!("Invalid address format: {}", addr));
        }
        let addr_str = &addr[1..addr.len()-1];
        if addr_str.starts_with("0x") {
            usize::from_str_radix(&addr_str[2..], 16)
        } else {
            addr_str.parse::<usize>()
        }.map_err(|e| format!("Failed to parse address: {}", e))
    }

    fn parse_immediate(&self, value: &str) -> Result<u8, String> {
        if value.starts_with("0b") {
            u8::from_str_radix(&value[2..], 2)
        } else if value.starts_with("0x") {
            u8::from_str_radix(&value[2..], 16)
        } else {
            value.parse::<u8>()
        }.map_err(|e| format!("Failed to parse immediate value: {}", e))
    }

    pub fn execute(&mut self, instruction: &str) -> Result<bool, String> {
        let parts: Vec<&str> = instruction.split(';').next().unwrap().trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(true);
        }


        match parts[0] {
            "LOAD" => {
                if parts.len() < 3 {
                    return Err(format!("LOAD instruction must have at least 3 parts: {}", instruction));
                }
                let reg_index = self.parse_register(parts[1])?;
                let address = self.parse_address(parts[2])?;
                self.registers[reg_index] = self.ram.read(address)?;
                if self.verbose {
                    println!("LOAD: Loaded R{} = {:08b}", reg_index, self.registers[reg_index]);
                }
            }
            "VER" => {
                if parts.len() != 3 {
                    return Err(format!("VER instruction must have 3 parts: {}", instruction));
                }
                let verbose_value = self.parse_immediate(parts[2])?;
                if verbose_value > 1 {
                    return Err(format!("VER instruction must contain 0 or 1: {}", instruction));
                }
                self.verbose = verbose_value == 1;
                if self.verbose {
                    println!("VER: VER = {} -> SET", verbose_value);
                }
            }
            "ADD" => {
                if parts.len() != 4 {
                    return Err(format!("ADD instruction must have 4 parts: {}", instruction));
                }
                let r1 = self.registers[
                    self.parse_register(parts[1])?
                ];
                let r2 = self.registers[
                    self.parse_register(parts[2])?
                ];
                let r3_index = self.parse_register(parts[3])?;
                self.registers[r3_index] = r1.wrapping_add(r2);
                if self.verbose {
                    println!("ADD: {} + {} = {} -> {:08b}", parts[1], parts[2], parts[3], self.registers[r3_index]);
                }
            }
            "STORE" => {
                if parts.len() != 3 {
                    return Err(format!("STORE instruction must have 3 parts: {}", instruction));
                }
                let reg_index = self.parse_register(parts[1])?;
                let address = self.parse_address(parts[2])?;
                self.ram.write(address, self.registers[reg_index])?;
                if self.verbose {
                    println!("STORE: R{} stored at {} -> {:08b}", parts[1], parts[2], self.registers[reg_index]);
                }
            }
            "INIT" => {
                if parts.len() != 4 || parts[2] != "=" {
                    return Err(format!("INIT instruction must be in format INIT [address] = [value]: {}", instruction));
                }
                let address = self.parse_address(parts[1])?;
                let value = self.parse_immediate(parts[3])?;
                self.ram.write(address, value)?;
                if self.verbose {
                    println!("INIT: Set memory {} to {:08b}", parts[1], value);
                }
            }
            "CLEAR" => {
                if parts.len() != 2 {
                    return Err(format!("CLEAR instruction must have 2 parts: {}", instruction));
                }
                if parts[1].starts_with('R') {
                    let reg = self.parse_register(parts[1])?;
                    self.registers[reg] = 0;
                } else {
                    let address = self.parse_address(parts[1])?;
                    self.ram.write(address, 0)?;
                }
                if self.verbose {
                    println!("CLEAR: Cleared memory at {}", parts[1]);
                }
            }
            "OUT" => {
                if parts.len() != 2 {
                    return Err(format!("OUT instruction must have 2 parts: {}", instruction));
                }
                let out_message = if parts[1].starts_with('R') {
                    let reg =self.parse_register(parts[1])?;
                    format!("REG R{}={:08b}", reg, self.registers[reg])
                } else {
                    let value = self.parse_immediate(parts[1])?;
                    format!("Value {:08b}", value)
                };
                println!("OUT: {}", out_message);
            }
            "HALT" => return Ok(false),
            "AND" | "OR" | "NAND" | "NOR" | "XOR" => {
                if parts.len() != 4 {
                    return Err(format!("{} instruction must have 4 parts: {}", parts[0], instruction));
                }
                let r1 = self.registers[
                    self.parse_register(parts[1])?
                ];
                let r2 = self.registers[
                    self.parse_register(parts[2])?
                ];
                let r3_index =  self.parse_register(parts[3])?;
                self.registers[r3_index] = match parts[0] {
                    "AND" => LogicGates::and(r1, r2),
                    "OR" => LogicGates::or(r1, r2),
                    "NAND" => LogicGates::nand(r1, r2),
                    "NOR" => LogicGates::nor(r1, r2),
                    "XOR" => LogicGates::xor(r1, r2),
                    _ => unreachable!(),
                };
                if self.verbose {
                    println!("{}: R{} {} R{} = R{} -> {:08b}", parts[0], parts[1], parts[0], parts[2], parts[3], self.registers[r3_index]);
                }
            }
            "NOT" => {
                if parts.len() != 3 {
                    return Err(format!("NOT instruction must have 3 parts: {}", instruction));
                }
                let reg = self.registers[
                    self.parse_register(parts[1])?
                    ];
                let target_reg_index =  self.parse_register(parts[2])?;
                self.registers[target_reg_index] = LogicGates::not(reg);
                if self.verbose {
                    println!("NOT: R{} -> R{} -> {:08b}", parts[1], parts[2], self.registers[target_reg_index]);
                }
            }
            "MOV" => {
                if parts.len() < 3 {
                    return Err(format!("MOV instruction must have at least 3 parts: {}", instruction));
                }
                let value = if parts[2].starts_with('R') {
                    let reg =self.parse_register(parts[2])?;
                    self.registers[reg]
                } else if parts[2].starts_with('['){
                    let address = self.parse_address(parts[2])?;
                    self.ram.read(address)?
                }else {
                    self.parse_immediate(parts[2])?
                };            

                if parts[1].starts_with('R'){
                    let reg_index = self.parse_register(parts[1])?;
                    self.registers[reg_index] = value;
    
                    if self.verbose {
                        println!("MOV: MOVED R{} = {:08b}", reg_index, self.registers[reg_index]);
                    }
                } else if parts[1].starts_with('['){ 
                    let address = self.parse_address(parts[1])?;
                    self.ram.write(address, value)?;
                    if self.verbose {
                        println!("MOV: MOVED {} stored with -> {:08b}", parts[1], value);
                    }
                }
                
            }
            "QMOV" => {
                if parts.len() != 3 {
                    return Err(format!("QMOV instruction must have 3 parts: {}", instruction));
                }
                let src = parts[2];
                let dst = parts[1];

                if src.starts_with('R') && dst.starts_with('[') {
                    // Move from register to RAM
                    println!("1");
                    let reg_index = self.parse_register(src)?;
                    let address = self.parse_address(dst)?;
                    self.ram.write(address, self.registers[reg_index])?;
                    self.registers[reg_index] = 0b00000000; // Clear the register
                } else if src.starts_with('[') && dst.starts_with('R') {
                    // Move from RAM to register
                    let address = self.parse_address(src)?;
                    let reg_index = self.parse_register(dst)?;
                    self.registers[reg_index] = self.ram.read(address)?;
                    self.ram.write(address, 0b00000000)?; // Clear the RAM value
                } else {
                    return Err(format!("Invalid QMOV instruction: {}", instruction));
                }
                if self.verbose {
                    println!("QMOV: Moved value from {} to {}", src, dst);
                }
            }
            "IF" => {
                let (condition, then_clause) = instruction.split_once("THEN").ok_or("IF instruction must contain THEN")?;
                let condition = condition.trim();
                let (then_clause, else_clause) = then_clause.split_once("ELSE").map(|(t, e)| (t.trim(), Some(e.trim()))).unwrap_or((then_clause.trim(), None));

                let condition_parts: Vec<&str> = condition.split_whitespace().collect();
                if condition_parts.len() != 4 || !["==", "!=", ">", "<", ">=", "<="].contains(&condition_parts[2]) {
                    return Err(format!("Invalid IF condition: {}", condition));
                }

                let reg1_value = self.registers[
                    self.parse_register(parts[1])?
                    ];
                let value = if condition_parts[3].starts_with('R') {
                    self.registers[
                        self.parse_register(parts[3])?
                        ]
                } else if condition_parts[3].starts_with('[') {
                    self.ram.read( self.parse_address(parts[2])?)?
                } else {
                    self.parse_immediate(parts[3])?
                };

                let condition_met = match condition_parts[2] {
                    "==" => reg1_value == value,
                    "!=" => reg1_value != value,
                    ">" => reg1_value > value,
                    "<" => reg1_value < value,
                    ">=" => reg1_value >= value,
                    "<=" => reg1_value <= value,
                    _ => unreachable!(),
                };

                if condition_met {
                    if self.verbose {
                        println!("IF condition met: {} {} {:08b}, executing {}", condition_parts[1], condition_parts[2], value, then_clause);
                    }
                    self.execute(then_clause)?;
                } else if let Some(else_clause) = else_clause {
                    if self.verbose {
                        println!("IF condition not met, executing ELSE clause: {}", else_clause);
                    }
                    self.execute(else_clause)?;
                } else if self.verbose {
                    println!("IF condition not met: {} {} {:08b}, skipping THEN clause", condition_parts[1], condition_parts[2], value);
                }
            }
            _ => return Err(format!("Unknown instruction: {}", instruction)),
        }

        Ok(true)
    }
}