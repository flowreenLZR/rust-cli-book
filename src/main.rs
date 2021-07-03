
use structopt::StructOpt;

use std::fmt::{Display, Formatter, Result};
use std::io::BufRead;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    #[structopt(short = "p", long = "path")]
    path: std::path::PathBuf,
}

impl Display for Cli {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "This is the pattern: {} and this is the path: {:?}",
            self.pattern, self.path)
    }
}

fn main() {
    // Use the *from_args* method provided by *derive(StructOpt)"
    // to parse the input arguments.
    let args = Cli::from_args();

    println!("Pattern: {}", args.pattern);
    println!("Path (debug form): {:?}", args.path);

    println!("Cli args struct (debug): {:?}", args);

    println!("Cli args struct (display): {}", args);

    // "read_to_string" returns a "Result" struct which may contain
    // an OK value or an Err value.
    // "expect" then consumes the "self" object, returning the OK value
    // or panicking if the the result is an "Err".
    let content = std::fs::read_to_string(&args.path)
        .expect("Could not read the file!");

    // Iterate over content and match pattern.
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // Using std::fs::read_to_string is memory expensive because it reads the whole file
    // into memory.
    // BufReader should solve that.
    println!("Doing the same thing only this time using BufReader.");

    let file = std::fs::File::open(&args.path).expect("File could not pe opened!");
    let buf_reader = std::io::BufReader::new(file);

    for line in buf_reader.lines() {
        let string = line.expect("Iter invalid");
        if string.contains(&args.pattern) {
            println!("{}", string);
        }
    }
}
