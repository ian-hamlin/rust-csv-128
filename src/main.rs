#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Serialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u128>,
    over65s: i128,
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

fn example() -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // When writing records with Serde using structs, the header row is written
    // automatically.
    wtr.serialize(Record {
        city: "Southborough".to_string(),
        region: "MA".to_string(),
        country: "United States".to_string(),
        population: Some(340_282_366_920_938_463_463_374_607_431_768_211_455),
        over65s: 9_223_372_036_854_775_808,
    })?;
    wtr.flush()?;
    Ok(())
}