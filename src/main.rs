//! ned v0.1
//! A minimal line-based text editor in Rust modeled on ed. No syntax highlighting.
//! Usage: ned <filename>

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ned <filename>");
        std::process::exit(1);
    }
    let filename = args[1].clone();

    // Load file or start empty
    let mut buffer: Vec<String> = match fs::read_to_string(&filename) {
        Ok(contents) => contents.lines().map(String::from).collect(),
        Err(_) => Vec::new(),
    };

    // Cheat sheet
    println!("ned v0.1 - Commands:");
    println!("  [address]a      Append text after address");
    println!("  [address]i      Insert text before address");
    println!("  [range]c        Change lines in range");
    println!("  [range]d        Delete lines in range");
    println!("  [range]p        Print lines in range");
    println!("  w               Write buffer to file");
    println!("  q               Quit editor");
    println!();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line?;
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Quit
        if trimmed == "q" {
            break;
        }

        // Write
        if trimmed == "w" {
            fs::write(&filename, buffer.join("\n"))?;
            println!("{} lines written", buffer.len());
            continue;
        }

        // Parse address/range and command
        let first_alpha = trimmed.find(char::is_alphabetic).unwrap_or(0);
        let (addr_part, cmd_rest) = trimmed.split_at(first_alpha);
        let cmd = cmd_rest.chars().next().unwrap();
        let addr_part = addr_part.trim();

        // Determine range
        let (start, end) = if let Some(comma) = addr_part.find(',') {
            let a = &addr_part[..comma];
            let b = &addr_part[comma+1..];
            (parse_address(a, buffer.len()), parse_address(b, buffer.len()))
        } else if !addr_part.is_empty() {
            let n = parse_address(addr_part, buffer.len());
            (n, n)
        } else {
            (1, buffer.len())
        };

        match cmd {
            'p' => {
                // Print lines start..=end
                for i in start..=end {
                    if i >= 1 && i <= buffer.len() {
                        println!("{} {}", i, buffer[i-1]);
                    }
                }
            }
            'd' => {
                // Delete lines start..=end
                if start>=1 && end>=start && end<=buffer.len() {
                    for _ in start..=end {
                        buffer.remove(start-1);
                    }
                }
            }
            'a' => {
                // Append after end
                let pos = if end>=1 && end<=buffer.len() { end } else { buffer.len() };
                let mut insert = Vec::new();
                for entry in stdin.lock().lines() {
                    let txt = entry?;
                    if txt == "." { break; }
                    insert.push(txt);
                }
                for (i, txt) in insert.into_iter().enumerate() {
                    buffer.insert(pos + i, txt);
                }
            }
            'i' => {
                // Insert before start
                let pos = if start>=1 && start<=buffer.len() { start-1 } else { 0 };
                let mut insert = Vec::new();
                for entry in stdin.lock().lines() {
                    let txt = entry?;
                    if txt == "." { break; }
                    insert.push(txt);
                }
                for (i, txt) in insert.into_iter().enumerate() {
                    buffer.insert(pos + i, txt);
                }
            }
            'c' => {
                // Change: delete then insert
                if start>=1 && end>=start && end<=buffer.len() {
                    for _ in start..=end {
                        buffer.remove(start-1);
                    }
                    let mut insert = Vec::new();
                    for entry in stdin.lock().lines() {
                        let txt = entry?;
                        if txt == "." { break; }
                        insert.push(txt);
                    }
                    for (i, txt) in insert.into_iter().enumerate() {
                        buffer.insert(start-1 + i, txt);
                    }
                }
            }
            _ => {
                eprintln!("Unknown command: {}", cmd);
            }
        }
    }

    Ok(())
}

/// Parse an address: number or "$" for last line
fn parse_address(s: &str, last: usize) -> usize {
    if s == "$" { last }
    else { s.parse().unwrap_or(1) }
}
