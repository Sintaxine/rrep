use std::fs;

use std::io;



fn main() {
    let mut buffer = String::new();
    println!("Please enter the name of the file to open below.");
    
    io::stdin().read_line(&mut buffer).expect("Error occured!");

    let buffer = String::from(buffer.trim());
 
    let lines = read_line_by_line(buffer);

    for line in lines {
      let formattedline = parse_line(line); 
      
      println!("{}", formattedline);
    }

}



fn read_line_by_line(filename: String) -> Vec<String>{
  let mut result = Vec::new();

  for line in fs::read_to_string(filename).unwrap().lines(){
    result.push(line.to_string())
  }

  result
} 


fn parse_line(line: String) -> String {
  let red = "\x1b[31m"; // red ansi code;
  let bold = "\x1b[1m";
  let reset = "\x1b[0m";
  let _linereset: String = format!("{}{}", line , "\x1b[0m");
  if line.starts_with("#") {
    let linereset = line.trim_start_matches("#").trim_start();
    return format!("{}{}{}",red, linereset, reset);

  } else {
    let words = read_word_by_word(line);
    let mut result: Vec<String> = Vec::new();
    for word in words {
      if word.starts_with("**") && word.ends_with("**") {
        let word = word.trim_start_matches("**").trim_start().trim_end_matches("**").trim_end();
        result.push(format!("{}{}{}", bold, word, reset));
      } else {
        result.push(format!("{}", word)); 
      }
    }

    let resultstring = result.join(" ");

    resultstring 
  }

}


fn read_word_by_word(line: String) -> Vec<String>{
  let mut result: Vec<String> = Vec::new();

  for word in line.split_whitespace(){
  result.push(word.to_string())
  }

  result
}
