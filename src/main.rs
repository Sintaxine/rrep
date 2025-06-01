use std::fs;
use std::io;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crossterm::terminal::size;

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

    println!("{:?}", size());
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
                                chars.next(); // consume ')' // "no consume my dick instead" - FKF
                                break;
                            }
                            url.push(chars.next().unwrap());
                        }

                        // 🚨 RENDER IMAGE HERE
                        if let Ok(img) = image::open(&url) {
                            // Get terminal size
                            let (width, height) = size().unwrap();
                            
                            // Calculate image size (usein n% of terminal width and heigh. for now 40 is what I'm going wth)
                            let img_width = (width as u32 ) / 2;  // divide by 2 because terminal cells are around that aspect ration. unless it's some fancy shit
                            //or else it's too widwe
                            let img_height = (height as u32 );
                            println!("{}{}", img_width, img_height);

                            //IDK the scaling bug and I give up kys
                            
                            // Resize image to fit terminal
                            let resized = img.resize_exact(img_width, img_height, image::imageops::FilterType::Lanczos3); //the lancoz shit was from the internet IDK what this shit does
                            
                            // Convert to RGB (not RGBA)
                            let rgb = resized.to_rgb8();
                            
                            // Encode as base64
                            let b64 = BASE64.encode(rgb.as_raw());
                            
                            // Construct and print Kitty escape sequence
                            // Using f=24 for 24-bit RGB format /// IIRC you can compress the size here 
                            print!("\x1b_Ga=T,f=24,i=1,s={},v={};{}\x1b\\",
                                rgb.width(),
                                rgb.height(),
                                b64
                            );
                            io::stdout().flush().unwrap();
                        }

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
