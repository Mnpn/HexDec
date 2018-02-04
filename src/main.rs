#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::error::Error;

fn main() {
    // If any error would occur in inner_main(), print the error.
    if let Err(err) = inner_main() {
        eprintln!("{}", err);
    }
}

fn inner_main() -> Result<(), Box<Error>> {
    // clap app creation, with macros that read project information from Cargo.toml.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("hex")
            .help("Hex to convert to decimal.")
            .required(true)) // Make argument required.
        .get_matches();

    // New hex variable
    let mut hex = matches.value_of("hex").unwrap();

    // If the hex string contains a #, remove it and then convert hex to dec.
    if hex.starts_with("#") {
        hex = &hex[1..];
    }
    let dec = u64::from_str_radix(hex, 16)?;

    // Print the result.
    println!("{}", dec);

    // We're done here. Good job, code!
    Ok(())
}