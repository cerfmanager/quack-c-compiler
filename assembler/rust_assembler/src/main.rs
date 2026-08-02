use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: quasm <input.qasm> [output.duck]");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        input_path.trim_end_matches(".qasm").to_string() + ".duck"
    };
    println!("path is being read");
    let source = read_file(input_path)?;
    println!("file has been read");
    let jump_table = build_jump_table(&source);
    let program_bin = assemble(&source, &jump_table)?;

    let mut file = File::create(&output_path)?;
    file.write_all(&program_bin)?;

    println!(
        "Assembled {} -> {} ({} bytes)",
        input_path,
        output_path,
        program_bin.len()
    );
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Build a map of label -> program counter (in bytes, counting 6-byte instructions)
fn build_jump_table(source: &str) -> HashMap<String, u16> {
    let mut jump_map = HashMap::new();
    let mut pc = 0u16;

    for line in source.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        // Label definition (ends with ':')
        if line.ends_with(":") {
            let label = line[..line.len() - 1].trim();
            jump_map.insert(label.to_string(), pc);
            continue;
        }

        // Regular instruction: 6 bytes
        pc += 6;
    }

    jump_map
}

fn assemble(source: &str, jump_table: &HashMap<String, u16>) -> io::Result<Vec<u8>> {
    let mut program: Vec<u8> = Vec::new();

    for line in source.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        // Skip labels
        if line.ends_with(":") {
            continue;
        }

        let instruction = parse_instruction(line, jump_table).unwrap_or_else(|e| {
            eprintln!("Error parsing: {}", line);
            eprintln!("  {}", e);
            std::process::exit(1);
        });

        program.extend_from_slice(&instruction);
    }

    Ok(program)
}

fn parse_instruction(line: &str, jump_table: &HashMap<String, u16>) -> Result<[u8; 6], String> {
    let parts: Vec<&str> = line
        .split(|c: char| c.is_whitespace() || c == ',' || c == '$' || c == '&')
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return Err("Empty instruction".to_string());
    }

    let mut instr = [0u8; 6];
    instr[0] = match parts[0] {
        // Register moves (0x01-0x03)
        "rrmovb" => 0x01,
        "rrmovw" => 0x02,
        "rrmovd" => 0x03,

        // Immediate loads (0x04-0x06)
        "irmovb" => 0x04,
        "irmovw" => 0x05,
        "irmovd" => 0x06,

        // Memory loads (0x07-0x09)
        "mrmovb" => 0x07,
        "mrmovw" => 0x08,
        "mrmovd" => 0x09,

        // Memory stores (0x0A-0x0C)
        "rmmovb" => 0x0A,
        "rmmovw" => 0x0B,
        "rmmovd" => 0x0C,

        // Arithmetic (0x0D-0x11)
        "add" => 0x0D,
        "sub" => 0x0E,
        "inc" => 0x0F,
        "dec" => 0x10,
        "clr" => 0x11,

        // Logical register-register (0x12-0x14)
        "rand" => 0x12,
        "ror" => 0x13,
        "rxor" => 0x14,

        // Logical register-immediate (0x15-0x17)
        "iand" => 0x15,
        "ior" => 0x16,
        "ixor" => 0x17,

        // Shifts (0x18-0x1B)
        "shl" => 0x18,
        "shr" => 0x19,
        "sar" => 0x1A,
        "sal" => 0x1B,

        // Control flow (0x1C-0x25)
        "cmp" => 0x1C,
        "jmp" => 0x1D,
        "je" => 0x1E,
        "jne" => 0x1F,
        "jg" => 0x20,
        "jl" => 0x21,
        "jge" => 0x22,
        "jle" => 0x23,
        "ja" => 0x24,
        "jb" => 0x25,

        // Stack/procedure (0x26-0x29)
        "pushw" => 0x26,
        "popw" => 0x27,
        "call" => 0x28,
        "ret" => 0x29,

        // Output (0x2A)
        "outc" => 0x2A,

        // Halt (0x2B)
        "halt" => 0x2B,

        // Unary arithmetic (0x2C)
        "neg" => 0x2C,

        // Unary logical (0x2D)
        "bitcomp" => 0x2D,

        // Immediate to memory (0x2E-0x2F)
        "immovb" => 0x2E,
        "immovw" => 0x2F,

        // Indirect memory (0xAA, 0xDD)
        "rmrmovd" => 0xAA,
        "rrmmovd" => 0xDD,

        _ => return Err(format!("Unknown instruction: {}", parts[0])),
    };

    // Parse operands based on instruction type
    match parts[0] {
        // Register-to-register moves: rrmov* rSRC, rDST
        "rrmovb" | "rrmovw" | "rrmovd" => {
            if parts.len() < 3 {
                return Err("rrmov* requires two register operands".to_string());
            }
            instr[1] = parse_register(parts[2])?; // destination
            instr[3] = parse_register(parts[1])?; // source (b3)
        }

        // Immediate loads: irmov* $imm, rDST
        "irmovb" => {
            if parts.len() < 3 {
                return Err("irmovb requires immediate and register".to_string());
            }
            instr[1] = parse_register(parts[2])?;
            let imm = parse_immediate(parts[1])?;
            instr[2] = (imm & 0xFF) as u8;
        }

        "irmovw" => {
            if parts.len() < 3 {
                return Err("irmovw requires immediate and register".to_string());
            }
            instr[1] = parse_register(parts[2])?;
            let imm = parse_immediate(parts[1])?;
            instr[2] = (imm & 0xFF) as u8;
            instr[3] = ((imm >> 8) & 0xFF) as u8;
        }

        "irmovd" => {
            if parts.len() < 3 {
                return Err("irmovd requires immediate and register".to_string());
            }
            instr[1] = parse_register(parts[2])?;
            let imm = parse_immediate_32(parts[1])?;
            instr[2] = (imm & 0xFF) as u8;
            instr[3] = ((imm >> 8) & 0xFF) as u8;
            instr[4] = ((imm >> 16) & 0xFF) as u8;
            instr[5] = ((imm >> 24) & 0xFF) as u8;
        }

        // Memory loads: mrmov* $addr, rDST
        "mrmovb" | "mrmovw" | "mrmovd" => {
            if parts.len() < 3 {
                return Err(format!("{} requires address and register", parts[0]));
            }
            instr[1] = parse_register(parts[2])?;
            let addr = parse_address(parts[1])?;
            instr[2] = (addr & 0xFF) as u8;
            instr[3] = ((addr >> 8) & 0xFF) as u8;
        }

        // Memory stores: rmmov* $addr, rSRC
        "rmmovb" | "rmmovw" | "rmmovd" => {
            if parts.len() < 3 {
                return Err(format!("{} requires address and register", parts[0]));
            }
            instr[1] = parse_register(parts[2])?;
            let addr = parse_address(parts[1])?;
            instr[2] = (addr & 0xFF) as u8;
            instr[3] = ((addr >> 8) & 0xFF) as u8;
        }

        // Indirect memory: rmrmovd rADDR, rDST or rrmmovd rSRC, rADDR
        "rmrmovd" | "rrmmovd" => {
            if parts.len() < 3 {
                return Err(format!("{} requires two register operands", parts[0]));
            }
            instr[1] = parse_register(parts[1])?;
            instr[2] = parse_register(parts[2])?; // b2
        }

        // Arithmetic register-register: add rSRC, rDST | sub rSRC, rDST
        "add" | "sub" => {
            if parts.len() < 3 {
                return Err(format!("{} requires two register operands", parts[0]));
            }
            instr[1] = parse_register(parts[1])?; // source (ra)
            instr[2] = parse_register(parts[2])?; // destination (b2)
        }

        // Arithmetic unary: inc/dec/clr rX | neg rX | bitcomp rX
        "inc" | "dec" | "clr" | "neg" | "bitcomp" => {
            if parts.len() < 2 {
                return Err(format!("{} requires a register operand", parts[0]));
            }
            instr[1] = parse_register(parts[1])?;
        }

        // Logical register-register: rand/ror/rxor rSRC, rDST
        "rand" | "ror" | "rxor" => {
            if parts.len() < 3 {
                return Err(format!("{} requires two register operands", parts[0]));
            }
            instr[1] = parse_register(parts[1])?; // source (ra)
            instr[2] = parse_register(parts[2])?; // destination (b2)
        }

        // Logical register-immediate: iand/ior/ixor $imm, rX
        "iand" | "ior" | "ixor" => {
            if parts.len() < 3 {
                return Err(format!("{} requires immediate and register", parts[0]));
            }
            instr[1] = parse_register(parts[2])?;
            let imm = parse_immediate_32(parts[1])?;
            instr[2] = (imm & 0xFF) as u8;
            instr[3] = ((imm >> 8) & 0xFF) as u8;
            instr[4] = ((imm >> 16) & 0xFF) as u8;
            instr[5] = ((imm >> 24) & 0xFF) as u8;
        }

        // Shifts: shl/shr/sar/sal $amount, rX
        "shl" | "shr" | "sar" | "sal" => {
            if parts.len() < 3 {
                return Err(format!("{} requires amount and register", parts[0]));
            }
            instr[1] = parse_register(parts[2])?;
            let amount = parse_immediate_32(parts[1])?;
            instr[2] = (amount & 0xFF) as u8;
            instr[3] = ((amount >> 8) & 0xFF) as u8;
            instr[4] = ((amount >> 16) & 0xFF) as u8;
            instr[5] = ((amount >> 24) & 0xFF) as u8;
        }

        // Compare: cmp rA, rB
        "cmp" => {
            if parts.len() < 3 {
                return Err("cmp requires two register operands".to_string());
            }
            instr[1] = parse_register(parts[1])?; // ra
            instr[2] = parse_register(parts[2])?; // b2
        }

        // Jumps: jmp/je/jne/jg/jl/jge/jle/ja/jb label
        "jmp" | "je" | "jne" | "jg" | "jl" | "jge" | "jle" | "ja" | "jb" => {
            if parts.len() < 2 {
                return Err(format!("{} requires a label", parts[0]));
            }
            let label = parts[1];
            let addr = jump_table
                .get(label)
                .ok_or_else(|| format!("Undefined label: {}", label))?;
            instr[2] = (addr & 0xFF) as u8;
            instr[3] = ((addr >> 8) & 0xFF) as u8;
        }

        // Stack and procedure: pushw/popw rX | call label | ret
        "pushw" | "popw" => {
            if parts.len() < 2 {
                return Err(format!("{} requires a register operand", parts[0]));
            }
            instr[1] = parse_register(parts[1])?;
        }

        "call" => {
            if parts.len() < 2 {
                return Err("call requires a label".to_string());
            }
            let label = parts[1];
            let addr = jump_table
                .get(label)
                .ok_or_else(|| format!("Undefined label: {}", label))?;
            instr[2] = (addr & 0xFF) as u8;
            instr[3] = ((addr >> 8) & 0xFF) as u8;
        }

        "ret" | "halt" => {
            // No operands
        }

        // Output: outc rX
        "outc" => {
            if parts.len() < 2 {
                return Err("outc requires a register operand".to_string());
            }
            instr[1] = parse_register(parts[1])?;
        }

        // Immediate to memory: immovb/immovw $imm, $addr
        "immovb" => {
            if parts.len() < 3 {
                return Err("immovb requires immediate and address".to_string());
            }
            let imm = parse_immediate(parts[1])?;
            let addr = parse_address(parts[2])?;
            instr[2] = (addr & 0xFF) as u8;
            instr[3] = ((addr >> 8) & 0xFF) as u8;
            instr[4] = (imm & 0xFF) as u8;
        }

        "immovw" => {
            if parts.len() < 3 {
                return Err("immovw requires immediate and address".to_string());
            }
            let imm = parse_immediate(parts[1])?;
            let addr = parse_address(parts[2])?;
            instr[2] = (addr & 0xFF) as u8;
            instr[3] = ((addr >> 8) & 0xFF) as u8;
            instr[4] = (imm & 0xFF) as u8;
            instr[5] = ((imm >> 8) & 0xFF) as u8;
        }

        _ => {
            return Err(format!("Instruction not implemented: {}", parts[0]));
        }
    }

    Ok(instr)
}

/// Parse a register name like "r0", "r5", "rsp", "rbp"
fn parse_register(reg: &str) -> Result<u8, String> {
    match reg {
        "r0" => Ok(0),
        "r1" => Ok(1),
        "r2" => Ok(2),
        "r3" => Ok(3),
        "r4" => Ok(4),
        "r5" => Ok(5),
        "r6" | "rsp" => Ok(6),
        "r7" | "rbp" => Ok(7),
        _ => Err(format!("Invalid register: {}", reg)),
    }
}

/// Parse an immediate value (8 or 16-bit, stripped of '$')
fn parse_immediate(imm: &str) -> Result<u16, String> {
    if imm.starts_with("0x") || imm.starts_with("0X") {
        u16::from_str_radix(&imm[2..], 16).map_err(|_| format!("Invalid hex immediate: {}", imm))
    } else {
        imm.parse::<u16>()
            .map_err(|_| format!("Invalid immediate: {}", imm))
    }
}

/// Parse a 32-bit immediate value (stripped of '$')
fn parse_immediate_32(imm: &str) -> Result<u32, String> {
    if imm.starts_with("0x") || imm.starts_with("0X") {
        u32::from_str_radix(&imm[2..], 16).map_err(|_| format!("Invalid hex immediate: {}", imm))
    } else {
        imm.parse::<u32>()
            .map_err(|_| format!("Invalid immediate: {}", imm))
    }
}

/// Parse an address (16-bit, stripped of '$' or '&')
fn parse_address(addr: &str) -> Result<u16, String> {
    if addr.starts_with("0x") || addr.starts_with("0X") {
        u16::from_str_radix(&addr[2..], 16).map_err(|_| format!("Invalid hex address: {}", addr))
    } else {
        addr.parse::<u16>()
            .map_err(|_| format!("Invalid address: {}", addr))
    }
}
