use std::{env, fs};
use std::fs::File;
use std::process;
use std::io::{BufReader, BufRead};

fn main() {
   let args: Vec<String> = env::args().collect(); // Collect file path

   if args.len() != 2 {  // only two arguments are allowed
    eprintln!("Usage: read-rs <FILE_PATH>");
    process::exit(1)
   }

   let path = &args[1];

   // Check if the path points to a directory
   match fs::metadata(path) {
    Ok(metadata) if metadata.is_dir() => {
        eprintln!("This is a directory, not a file: {}", path);
        process::exit(1)
    }
    Ok(_) => {} // it's a file so proceed to opening file
    Err(error) => match error.kind() {  // Handle various possible errors
        std::io::ErrorKind::NotFound => {
            eprintln!("File not found: {}", path);
            process::exit(1)
        }
        std::io::ErrorKind::PermissionDenied => {
            eprintln!("Missing permissions to get metadata: {}", path);
            process::exit(1)
        }
        _ => {
            eprintln!("Unexpected error accessing {path}: {error}");
            process::exit(2)
        }
    }
   } 

   // validate that file is found and read-rs has permissions to read it
   let file_result = File::open(path);
   let file = match file_result {
    Ok(file) => file,
    Err(error) => {
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("File not found: {}", path);
                process::exit(1)
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Missing read permission: {}", path);
                process::exit(1)
            }
            _ => {
                eprintln!("Error opening file: {}", error);
                process::exit(2)
            }
        }
    }
   };

   // validate whether file is UTF-8
   let bytes = fs::read(path);
   let text = match bytes {
    Ok(text) => text,
    Err(error) => {
        eprintln!("Error reading file {}: {}", path, error);
        process::exit(1)
    }
   };

   match str::from_utf8(&text) {
    Ok(_) => {}  // file is utf-8 so proceed to reading
    Err(error) => {
        eprintln!("File {} is not valid UTF-8: {}", path, error);
        process::exit(1)
    }
   }

   

   // Read the file via streaming
   let reader = BufReader::new(file);
   for line in reader.lines() {
    match line {
        Ok(line) => println!("{}", line),
        Err(error) => match error.kind() {
            std::io::ErrorKind::InvalidData => {  // If file is not UTF-8, stop
                eprintln!("File is not valid UTF-8: {}", path);
                process::exit(1)
            }
            _ => {
                eprintln!("Unexpected error reading {path}: {error}");
                process::exit(2)
            }
        }
    }
   }
   process::exit(0)  // Successful 
}
