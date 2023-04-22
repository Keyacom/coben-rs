mod cli;
mod coben;
use coben::calc::Calculator;
use csv::ReaderBuilder;
use cli::arg::CliOptions;
use clap::Parser; // required trait
use std::io::stdin as get_stdin;

fn main() {
    CliOptions::parse();
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(get_stdin());
    let records: Vec<_> = reader
        .records()
        .filter_map(|e| e.ok())
        .take(2)
        .collect();
    let mut calc = Calculator::new(
        records[0].iter().map(|e| e.parse::<u32>().unwrap()).collect(),
        records[1].iter().map(|e| e.parse::<u32>().unwrap()).collect()
    ).unwrap();
    calc.calculate();
    println!("{}", calc);
}
