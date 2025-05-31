use std::fs;
use std::io;

// ANSI escape codes
const RED_ANSI: &str = "\x1b[31m";
const BOLD_ANSI: &str = "\x1b[1m";
const ITALIC_ANSI: &str = "\x1b[3m";
const RESET_ANSI: &str = "\x1b[0m";
const BLUE_ANSI: &str = "\x1b[34m";
const GREEN_ANSI: &str = "\x1b[32m";
const LINK_ANSI: &str = "\x1b[34;4m"; // Blue with underline

fn main() -> io::Result<()> {
    println!("Please enter the name of the file to open below.");
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();

    let lines = read_lines(filename)?;

    for line in lines {
        let formatted_line = parse_line(line);
        println!("{}", formatted_line);
    }

    Ok(())
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|content| {
        content.lines().map(String::from).collect()
    })
}

fn parse_line(line: String) -> String {
    if line.starts_with("###") {
        format!("{}{}{}", GREEN_ANSI, line.trim_start_matches("###").trim_start(), RESET_ANSI)
    } else if line.starts_with("##") {
        format!("{}{}{}", BLUE_ANSI, line.trim_start_matches("##").trim_start(), RESET_ANSI)
    } else if line.starts_with("#") {
        format!("{}{}{}", RED_ANSI, line.trim_start_matches("#").trim_start(), RESET_ANSI)
    } else {
        parse_formatted_text(&line)
    }
}

fn parse_formatted_text(line: &str) -> String {
    let mut result = String::new();
    let mut chars = line.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                // Start of a potential link
                let mut link_text = String::new();
                while let Some(&next) = chars.peek() {
                    if next == ']' {
                        chars.next(); // consume the ']'
                        break;
                    }
                    link_text.push(chars.next().unwrap());
                }
                
                // Check if this is followed by '(url)'
                if chars.next() == Some('(') {
                    let mut url = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == ')' {
                            chars.next(); // consume the ')'
                            break;
                        }
                        url.push(chars.next().unwrap());
                    }
                    
                    // Format as clickable link (OSC 8 escape sequence)
                    result.push_str(&format!(
                        "\x1b]8;;{}\x1b\\{}{}{}\x1b]8;;\x1b\\",
                        url, LINK_ANSI, link_text, RESET_ANSI
                    ));
                } else {
                    // Not a link, just regular text
                    result.push('[');
                    result.push_str(&link_text);
                    result.push(']');
                }
            }
            '*' => {
                // Handle bold/italic
                let next_char = chars.peek();
                if next_char == Some(&'*') {
                    chars.next(); // consume second '*'
                    let mut bold_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == '*' {
                            chars.next(); // consume '*'
                            if chars.peek() == Some(&'*') {
                                chars.next(); // consume second '*'
                                break;
                            } else {
                                bold_text.push('*');
                            }
                        } else {
                            bold_text.push(chars.next().unwrap());
                        }
                    }
                    result.push_str(&format!("{}{}{}", BOLD_ANSI, bold_text, RESET_ANSI));
                } else {
                    // Italic
                    let mut italic_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == '*' {
                            chars.next(); // consume '*'
                            break;
                        } else {
                            italic_text.push(chars.next().unwrap());
                        }
                    }
                    result.push_str(&format!("{}{}{}", ITALIC_ANSI, italic_text, RESET_ANSI));
                }
            }
            _ => {
                result.push(c);
            }
        }
    }
    
    result
}