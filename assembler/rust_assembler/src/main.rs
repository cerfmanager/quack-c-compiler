use std::fs::File;

use std::collections::HashMap;
use std::io::{Read, Write};

use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected file path")
    }
    let path = &args[1].to_owned();
    let output_path = args[1].trim_end_matches(".qasm").to_owned() + ".duck";

    let buffer = read_file(&path);

    let jump_table = set_up_jump_map(&buffer);

    let mut program_bin: Vec<u8> = Vec::new();

    for line_result in buffer.lines() {
        if line_result.is_empty() || line_result.starts_with("#") || line_result.ends_with(":") {
            continue;
        }

        let (op, ra, b2, b3) = parse_line(line_result, &jump_table);
        program_bin.extend([op, ra, b2, b3]);
    }
    let mut file = File::create(output_path)?;
    file.write_all(&program_bin)?;
    Ok(())
}

fn set_up_jump_map(buffer: &String) -> HashMap<String, u16> {
    let mut jump_map = HashMap::new();
    let mut program_counter = 0;
    for line in buffer.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("#") {
            continue;
        }
        if line.ends_with(":") {
            jump_map.insert(line[..line.len() - 1].to_string(), program_counter);
        } else {
            program_counter += 4;
        }
    }
    return jump_map;
}

fn read_file(path: &String) -> String {
    let path = &path;
    let mut file = match File::open(path) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    return buffer;
}

fn parse_line(line: &str, jump_map: &HashMap<String, u16>) -> (u8, u8, u8, u8) {
    let parts: Vec<&str> = line
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .collect();
    match parts[0] {
        "rrmovw" => (
            0x01,
            parse_register(&parts[1]),
            parse_register(&parts[2]),
            0x00,
        ),
        "irmovb" => {
            let imm = parse_imm(&parts[1]);
            (
                0x02,
                parse_register(&parts[2]),
                (imm & 0xFF) as u8,
                (imm >> 8) as u8,
            )
        }
        "mrmovw" => {
            let adrr = parse_address(&parts[2]);
            (
                0x03,
                parse_register(&parts[1]),
                (adrr & 0xFF) as u8,
                (adrr >> 8) as u8,
            )
        }
        "rmmovw" => {
            let adrr = parse_address(&parts[1]);
            (
                0x04,
                parse_register(&parts[2]),
                (adrr & 0xFF) as u8,
                (adrr >> 8) as u8,
            )
        }
        "irmovw" => {
            let imm = parse_imm(&parts[1]);
            (
                0x05,
                parse_register(&parts[2]),
                (imm & 0xFF) as u8,
                (imm >> 8) as u8,
            )
        }
        "mrmovb" => {
            let adrr = parse_address(&parts[2]);
            (
                0x06,
                parse_register(&parts[1]),
                (adrr & 0xFF) as u8,
                (adrr >> 8) as u8,
            )
        }
        "rmmovb" => {
            let adrr = parse_address(&parts[1]);
            (
                0x07,
                parse_register(&parts[2]),
                (adrr & 0xFF) as u8,
                (adrr >> 8) as u8,
            )
        }
        "mrmovbr" => (
            0x08,
            parse_register(&parts[1]),
            parse_register(&parts[2]),
            0x00,
        ),
        "rmmovbr" => (
            0x09,
            parse_register(&parts[1]),
            parse_register(&parts[2]),
            0x00,
        ),
        "addw" => (
            0x10,
            parse_register(&parts[1]),
            parse_register(&parts[2]),
            0x00,
        ),
        "subw" => (
            0x11,
            parse_register(&parts[1]),
            parse_register(&parts[2]),
            0x00,
        ),
        "incw" => (0x12, parse_register(&parts[1]), 0x00, 0x00),
        "decw" => (0x13, parse_register(&parts[1]), 0x00, 0x00),
        "clrw" => (0x14, parse_register(&parts[1]), 0x00, 0x00),
        "cmpw" => (
            0x15,
            parse_register(&parts[1]),
            parse_register(&parts[2]),
            0x00,
        ),
        "jmp" => {
            let addr = jump_map[parts[1]];
            (0x20, 0x00, (addr & 0xFF) as u8, (addr >> 8) as u8)
        }
        "je" => {
            let addr = jump_map[parts[1]];
            (0x21, 0x00, (addr & 0xFF) as u8, (addr >> 8) as u8)
        }
        "jne" => {
            let addr = jump_map[parts[1]];
            (0x22, 0x00, (addr & 0xFF) as u8, (addr >> 8) as u8)
        }
        "halt" => (0x23, 0x00, 0x00, 0x00),
        "pushw" => (0x30, parse_register(&parts[1]), 0x00, 0x00),
        "popw" => (0x31, parse_register(&parts[1]), 0x00, 0x00),
        "call" => {
            let addr = jump_map[parts[1]];
            (0x32, 0x00, (addr & 0xFF) as u8, (addr >> 8) as u8)
        }
        "ret" => (0x33, 0x00, 0x00, 0x00),
        "outc" => (0x40, parse_register(&parts[1]), 0x00, 0x00),
        _ => panic!("Unknown instruction: {}", parts[0]),
    }
}

fn parse_register(register: &str) -> u8 {
    register
        .trim_start_matches('r')
        .parse()
        .expect("invalid register")
}

fn parse_imm(immediate: &str) -> u16 {
    immediate
        .trim_start_matches('$')
        .parse()
        .expect("invalid immediate")
}

fn parse_address(address: &str) -> u16 {
    address
        .trim_start_matches('&')
        .parse()
        .expect("invalid address")
}
