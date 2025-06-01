use std::fs;
use std::io;
use viuer::{print_from_file};

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
    fs::read_to_string(filename).map(|content| content.lines().map(String::from).collect())
}

fn parse_line(line: String) -> String {
    if line.starts_with("###") {
        format!(
            "{}{}{}",
            GREEN_ANSI,
            line.trim_start_matches("###").trim_start(),
            RESET_ANSI
        )
    } else if line.starts_with("##") {
        format!(
            "{}{}{}",
            BLUE_ANSI,
            line.trim_start_matches("##").trim_start(),
            RESET_ANSI
        )
    } else if line.starts_with("#") {
        format!(
            "{}{}{}",
            RED_ANSI,
            line.trim_start_matches("#").trim_start(),
            RESET_ANSI
        )
    } else {
        parse_formatted_text(&line)
    }
}

fn parse_formatted_text(line: &str) -> String {
    let mut result = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '!' => {
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume '['
                    let mut alt_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == ']' {
                            chars.next(); // consume ']'
                            break;
                        }
                        alt_text.push(chars.next().unwrap());
                    }

                    if chars.next() == Some('(') {
                        let mut url = String::new();
                        while let Some(&next) = chars.peek() {
                            if next == ')' {
                                chars.next(); // consume ')'
                                break;
                            }
                            url.push(chars.next().unwrap());
                        }

                        // 🚨 RENDER IMAGE HERE
                        let config = viuer::Config {
                            width: Some(40),  // in terminal *cells*, not pixels
                            height: Some(20), // adjust to whatever doesn't wreck your layout
                            ..Default::default()
                        };
                        let _ = print_from_file(&url, &config);

                        // Fallback text if image fails
                        result.push_str(&format!(
                            "{}[Image: {}]{}{}",
                            ITALIC_ANSI, alt_text, BLUE_ANSI, url
                        ));
                        result.push_str(RESET_ANSI);
                    } else {
                        result.push_str(&format!("![{}]", alt_text));
                    }
                } else {
                    result.push('!');
                }
            }

            '[' => {
                let mut link_text = String::new();
                while let Some(&next) = chars.peek() {
                    if next == ']' {
                        chars.next();
                        break;
                    }
                    link_text.push(chars.next().unwrap());
                }

                if chars.next() == Some('(') {
                    let mut url = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == ')' {
                            chars.next();
                            break;
                        }
                        url.push(chars.next().unwrap());
                    }

                    result.push_str(&format!(
                        "\x1b]8;;{}\x1b\\{}{}{}\x1b]8;;\x1b\\",
                        url, LINK_ANSI, link_text, RESET_ANSI
                    ));
                } else {
                    result.push('[');
                    result.push_str(&link_text);
                    result.push(']');
                }
            }

            '*' => {
                let next_char = chars.peek();
                if next_char == Some(&'*') {
                    chars.next();
                    let mut bold_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == '*' {
                            chars.next();
                            if chars.peek() == Some(&'*') {
                                chars.next();
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
                    let mut italic_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == '*' {
                            chars.next();
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
