use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <bf-file>", args[0]);
        std::process::exit(1);
    }

    let program = std::fs::read_to_string(&args[1]).expect("Failed to read file");
    let commands: Vec<u8> = program
        .bytes()
        .filter(|b| matches!(b, b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']'))
        .collect();

    let bracket_map = match build_bracket_map(&commands) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut mem = vec![0u8; 30000];
    let mut ptr: usize = 0;
    let mut loc = 0;

    while loc < commands.len() {
        let cmd = commands[loc];
        match cmd {
            b'>' => ptr = (ptr + 1).min(29999),
            b'<' => ptr = ptr.saturating_sub(1),
            b'+' => mem[ptr] = mem[ptr].wrapping_add(1),
            b'-' => mem[ptr] = mem[ptr].wrapping_sub(1),
            b'.' => {
                io::stdout().write_all(&[mem[ptr]]).unwrap();
                io::stdout().flush().unwrap();
            }
            b',' => {
                let mut buf = [0u8];
                if io::stdin().read_exact(&mut buf).is_ok() {
                    mem[ptr] = buf[0];
                } else {
                    mem[ptr] = 0;
                }
            }
            b'[' => {
                if mem[ptr] == 0 {
                    loc = bracket_map[loc].unwrap();
                }
            }
            b']' => {
                if mem[ptr] != 0 {
                    loc = bracket_map[loc].unwrap();
                }
            }
            _ => unreachable!(),
        }
        loc += 1;
    }
}

fn build_bracket_map(commands: &[u8]) -> Result<Vec<Option<usize>>, &'static str> {
    let mut map = vec![None; commands.len()];
    let mut stack = Vec::new();

    for (i, &cmd) in commands.iter().enumerate() {
        match cmd {
            b'[' => stack.push(i),
            b']' => {
                if let Some(start) = stack.pop() {
                    map[start] = Some(i);
                    map[i] = Some(start);
                } else {
                    return Err("Unmatched ']'");
                }
            }
            _ => (),
        }
    }

    if !stack.is_empty() {
        return Err("Unmatched '['");
    }

    Ok(map)
}
