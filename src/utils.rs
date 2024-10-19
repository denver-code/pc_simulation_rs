pub fn parse_address(addr: &str) -> Result<usize, String> {
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

pub fn parse_immediate(value: &str) -> Result<u8, String> {
    if value.starts_with("0b") {
        u8::from_str_radix(&value[2..], 2)
    } else if value.starts_with("0x") {
        u8::from_str_radix(&value[2..], 16)
    } else {
        value.parse::<u8>()
    }.map_err(|e| format!("Failed to parse immediate value: {}", e))
}

pub fn parse_register(reg: &str) -> Result<usize, String> {
    let reg = reg.trim_end_matches(',');
    if !reg.starts_with('R') {
        return Err(format!("Invalid register format: {}", reg));
    }
    reg[1..].parse::<usize>()
        .map_err(|e| format!("Failed to parse register number: {}", e))
}