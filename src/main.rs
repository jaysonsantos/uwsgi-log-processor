extern crate csv;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate glob;
#[macro_use]
extern crate nom;

mod errors;
mod parser;
mod process;

quick_main!(process::process);
