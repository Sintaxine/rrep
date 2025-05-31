use std::fs;
use std::io;

const REDANSI: &str = "\x1b[31m"; // red ANSI code
const BOLDANSI: &str = "\x1b[1m"; // bold ANSI code
const ITALICANSI: &str = "\x1b[3m"; // italic ANSI code
const RESETANSI: &str = "\x1b[0m";

fn main() {
    let mut buffer = String::new();
    println!("Please enter the name of the file to open below.");

    io::stdin().read_line(&mut buffer).expect("Error occured!");

    let buffer = String::from(buffer.trim());

    let lines = read_line_by_line(buffer);

    for line in lines {
        let formattedline = refactored_parse_line(line);

        println!("{}", formattedline);
    }
}

fn read_line_by_line(filename: String) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn read_word_by_word(line: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for word in line.split_whitespace() {
        result.push(word.to_string())
    }

    result
}

fn refactored_parse_line(line: String) -> String {
    // Refactoring the parse_line function to also format inside of line bold and italic multiple words.

    // we're gonna keep the entire line formatting as there's no cleaning neede

    if line.starts_with("#") {
        let linereset = line.trim_start_matches("#").trim_start();
        return format!("{}{}{}", REDANSI, linereset, RESETANSI);
    }
    // whole line starts AND ends with ** ? make it all bold
    else if line.starts_with("**") && line.ends_with("**") {
        // trim the damn stars
        let trimmed = line.trim_start_matches("**").trim_end_matches("**").trim();
        return format!("{}{}{}", BOLDANSI, trimmed, RESETANSI);
    }
    // refactoring word by word

    // we're gonna check if the word starts with "**" or "*", if it does we're gonna find the pair, if the pair doesn't exist then we are going to not format.
    else {
        let words: Vec<String> = read_word_by_word(line);
        let mut result: Vec<String> = Vec::new();
        let mut i = 0;

        while i < words.len() {
            if words[i].starts_with("**") {
                let mut j = i;

                while j < words.len() - 1 && !words[j].ends_with("**") {
                    j += 1;
                }

                if j < words.len() && words[j].ends_with("**") {
                    // Collect the bold block
                    let mut bold_block = Vec::new();
                    for k in i..=j {
                        let word = words[k].trim_start_matches("**").trim_end_matches("**");
                        bold_block.push(word.to_string());
                    }

                    let joined = bold_block.join(" ");
                    result.push(format!("{}{}{}", BOLDANSI, joined, RESETANSI));
                    i = j + 1;
                    continue;
                }
            } // bold case
            
            else if words[i].starts_with("*") {
                let mut j = i;

                while j < words.len() - 1 && !words[j].ends_with("*") {
                    j += 1;
                }

                if j < words.len() && words[j].ends_with("*") {
                    // Collect the bold block
                    let mut bold_block = Vec::new();
                    for k in i..=j {
                        let word = words[k].trim_start_matches("*").trim_end_matches("*");
                        bold_block.push(word.to_string());
                    }

                    let joined = bold_block.join(" ");
                    result.push(format!("{}{}{}", ITALICANSI, joined, RESETANSI));

                    i = j + 1;
                    continue;
                }
            } // italic case


            result.push(words[i].clone());
            i += 1;
        }
        result.join(" ")
    }
}
