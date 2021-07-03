
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

// [^Option 5/6]
// fn main() {
// Option 5/6.
// "Box" is some kind of "unique_ptr" from C++?
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    // let content = std::fs::read_to_string(&args.path)
    //     .expect("Could not read the file!");

    // Iterate over content and match pattern.
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    // Using std::fs::read_to_string is memory expensive because it reads the whole file
    // into memory.
    // BufReader should solve that.

    let file = std::fs::File::open(&args.path);
    // Option 1.
    // "File::open" returns a "Result" which can be evaluated using "expect".
    // let file = file.expect("File could not be opened!");
    // let buf_reader = std::io::BufReader::new(file);
    // Option 2.
    // It also can be evaluated using a match.
    // let buf_reader : std::io::BufReader<std::fs::File>;
    // match file {
    //     Ok(handle) => {
    //         buf_reader = std::io::BufReader::new(handle);
    //         println!("File was opened properly!")
    //     },
    //     Err(msg) => {
    //         println!("File was not opened properly, error is: {}", msg);
    //         return; // This was needed otherwise rustc would report that
    //                 // buf_reader might be used uninitialized.
    //                 // @todo Is there a better way to halt execution?
    //     }
    // }
    // Option 3.
    // Just like option 2 but instead of print and return, panic.
    // let buf_reader = match file {
    //     Ok(handle) => {
    //         println!("File was opened properly!");
    //         std::io::BufReader::new(handle)
    //     },
    //     Err(msg) => {
    //         panic!("File was not opened properly, error is: {}", msg);
    //     }
    // };
    // Option 4.
    // Use "Result::unwrap". Also panics.
    // Shortcut for option 3.
    // let buf_reader = std::io::BufReader::new(file.unwrap());
    // Option 5.
    // Just like option 2 but with nicer return. This changes signature of main
    // and the return value.
    // let buf_reader = match file {
    //     Ok(file) => { std::io::BufReader::new(file) },
    //     Err(msg) => { return Err(msg.into()); }
    // };
    // Option 6.
    // Shortcut for option 5.
    // Although File::open may return a std::io::Error and main returns std::error::Error,
    // "?" expands to code that converts between error types. Kind of what the "Err" branch does
    // in "Option 5"?
    let buf_reader = std::io::BufReader::new(file?);

    for line in buf_reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // Required for Option 5/6.
    Ok(())
}
