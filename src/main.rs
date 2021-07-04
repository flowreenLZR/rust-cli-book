
use structopt::StructOpt;

use anyhow::{Context, Result as AnyhowResult};

use std::fmt::{Display, Formatter, Result};
// @todo Why is `Write` needed for accessing `BufWriter`'s implementation of the trait's methods?
// @todo Why is importing `BufWriter` not enough to call its implementation of the `Writer` trait?
// See: https://github.com/flowreenLZR/rust-cli-book/issues/1
use std::io::{BufRead, Write};

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

// Simple struct with one member.
#[derive(Debug)]
struct CustomError(String);

// [^Option 5/6]
// fn main() {

// Option 5/6.
// "Box" is some kind of "unique_ptr" from C++?
// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

// Option 7.
// @todo Why isn't "Box" needed anymore?
// fn main() -> std::result::Result<(), CustomError> {

// Option 8.
fn main() -> AnyhowResult<()> {
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
    // let buf_reader = std::io::BufReader::new(file?);

    // Option 7.
    // Use a custom, user-defined error to provide a much more specific message.
    // All "?" operators must be prefixed with the "map_err" call.
    // That is unless the custom error implements the "From<"Error_Type">" trait where "Error_Type"
    // is the error type "?" was handling before. This is because "?" expands to code that does
    // error conversions as long as the necessary "From<E>" traits.
    // let file = file.map_err(|err| CustomError(
    //     format!("Error reading `{:?}`: {}", args.path, err)
    // ))?;
    // let buf_reader = std::io::BufReader::new(file);

    // Option 8.
    // @todo SOLVED Why "with_context", which is part of "anyhow::Context" can be invoked on "file", which is a "std::io::Result"?
    // Are there some type conversions being made?
    // No type conversions. "Context", a trait from "anyhow", is implemented by the "anyhow" library
    // for "std::result::Result".
    // @todo SOLVED "anyhow" implements "Context" for "std::result::Result" and not for "std::io::Result".
    // "std::io::Result<T>" is an alias for "std::result::Result<T, std::io::Error". Which means
    // that the "Context" implementation for "std::result::Result"
    // also applies to "std::io::Result".
    // This made me realize how cool Rust's Trait system is compared to C++'s inheritance system.
    // The trait system in rust is similar to the "extension" feature of C# in a way.

    let error_message = format!("Optoin 8: could not open file: {:?}!", args.path);
    // This will return an ANSIString that, when it's Display-ed, surrounds the text
    // with the required ANSI sequence that would make it red.
    let error_message = ansi_term::Colour::Red.paint(error_message);
    let file = file.with_context(|| error_message)?;
    let buf_reader = std::io::BufReader::new(file);

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    {
        // @todo Write to stdout from multiple threads. Stdout::write does not lock.
        // @todo Does `println!` lock? Test with long prints from different threads.
        // @todo Does Writer::write return error if another thread accesses the same object?
        // See: https://github.com/flowreenLZR/rust-cli-book/issues/3
        let stdout = std::io::stdout();
        let mut buf_writer = std::io::BufWriter::new(stdout);
    }
    // let stdout = std::io::stdout().lock(); // Error: lock does not consume the Stdout
        // object. Because of that, it needs to stay alive.
    let stdout = std::io::stdout();
    // Is it OK to lock here if the `for` loop might take a long time to finish?
    // One reason might be that the output of the `for` loop will not be interrupted by other
    // threads.
    // Creating the lock and the buffered writer inside the for loop does not seem to
    // make any sense because I don't see how that would make a difference.
    // @todo Create custom `BufferedStdout` that locks when flushing the internal buffer.
    let stdout_lock = stdout.lock();
    let mut buf_writer = std::io::BufWriter::new(stdout_lock);

    let mut match_index = 0;
    for line in buf_reader.lines() {
        // ^Option7
        // let line = line?;

        // Option 7.
        // Either this or implement "From<std::io::Error>" for "CustomError".
        // let line = line.map_err(|_| CustomError(
        //     format!("Could not read line from file!")
        // ))?;

        // Option 8.
        let line = line.with_context(|| format!("Could not read line from file!"))?;

        if line.contains(&args.pattern) {
            write!(buf_writer, "Match {}: {}\n", match_index, line)?;
            match_index += 1;
        }
    }
    // Although "BufWriter" calls "flush" when it's dropped, it's better to manually call it.
    // The reason for this is that if there are any errors during the dropping, they will be
    // ignored. Also, if the buffer is empty, the flush will not be performed.
    // @todo Test the attempt flush on drop behaviour.
    // See: https://github.com/flowreenLZR/rust-cli-book/issues/2
    buf_writer.flush()?;

    // Required for Option 5/6.
    Ok(())
}
