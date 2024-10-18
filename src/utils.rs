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