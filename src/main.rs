#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::{self, BufReader, Read};

const NUMBERS_IN_LINE:   usize = 32;
const LINE_HEX_CAPACITY: usize = 2 * NUMBERS_IN_LINE + NUMBERS_IN_LINE-1;
const LINE_CAPACITY:     usize = LINE_HEX_CAPACITY + 3 + NUMBERS_IN_LINE;

fn main() -> Result<(), Box<Error>> {
    // clap app creation, with macros that read project information from Cargo.toml.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("value")
            .help("Value to convert.")
            .required(true))
        .arg(Arg::with_name("dec")
            .help("Input decimal instead of the other way around.")
            .long("dec")
            .short("d"))
        .arg(Arg::with_name("file")
            .help("Inspect hex in file (or STDIN if file is -)")
            .long("file")
            .short("f")
            .conflicts_with("dec"))
        .get_matches();

    if matches.is_present("dec") {
        println!("{:X}", value_t_or_exit!(matches, "value", u64));
    } else {
        let mut input = matches.value_of("value").unwrap();

        if matches.is_present("file") {
            let file: Box<Iterator<Item = io::Result<u8>>> = if input == "-" {
                Box::new(io::stdin().bytes())
            } else {
                Box::new(BufReader::new(File::open(input)?).bytes())
            };

            let mut line = String::with_capacity(LINE_CAPACITY);
            let mut buffer = [0u8; NUMBERS_IN_LINE];

            fn flush(line: &mut String, buffer: &[u8]) {
                line.push_str(" | ");
                for byte in buffer {
                    if let 0x20...0x7e = *byte {
                        line.push(*byte as char);
                    } else {
                        line.push('.');
                    }
                }
                println!("{}", line);
            }

            let mut index = 0;

            for (i, byte) in file.enumerate() {
                let byte = byte?;
                index = i % NUMBERS_IN_LINE;

                if !line.is_empty() {
                    line.push(' ');
                }

                if i != 0 && index as usize == 0 {
                    flush(&mut line, &buffer);
                    buffer.iter_mut().for_each(|x| *x = 0);
                    line = String::with_capacity(LINE_CAPACITY);
                }

                buffer[index] = byte;

                write!(line, "{:02X}", byte).unwrap();
            }
            if !line.is_empty() {
                while line.len() < LINE_HEX_CAPACITY+1 {
                    line.push(' ');
                }
                flush(&mut line, &buffer[..index]);
            }
        } else {
            // If the hex string contains a # or 0x, remove it.
            if input.starts_with("#")  { input = &input[1..]; }
            if input.starts_with("0x") { input = &input[2..]; }

            println!("{}", u64::from_str_radix(input, 16)?);
        }
    }

    // We're done here. Good job, code!
    Ok(())
}
