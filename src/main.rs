use std::env;
use std::fs::File;
use std::process;
use std::io::{BufReader, BufRead};

fn main() {
   let args: Vec<String> = env::args().collect(); // Collect file path

   if args.len() != 2 {  // 
    eprintln!("Usage: read-rs <FILE_PATH>");
    process::exit(1)
   }

   let path = &args[1]; 

   let file_result = File::open(path);
   let file = match file_result {
    Ok(file) => file,
    Err(error) => {
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("File not found: {}", error);
                process::exit(1)
            }
            _ => {
                eprintln!("Error opening file: {}", error);
                process::exit(2)
            }
        }
    }
   };

   let reader = BufReader::new(file);
   for line in reader.lines() {
    match line {
        Ok(line) => println!("{}", line),
        Err(error) => {
            eprintln!("Error reading line: {}", error);
            process::exit(2)
        }
    }
   }
   process::exit(0)
}
