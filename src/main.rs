mod transitions_ics;

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    let tz_name = args.get(1).map(String::as_str).unwrap_or_else(|| {
        println!("ERROR: Please provide a time zone to generate a transitions .ics for!");
        exit(1)
    });
    println!("{}", transitions_ics::generate_transitions_ics(tz_name).unwrap());
}
