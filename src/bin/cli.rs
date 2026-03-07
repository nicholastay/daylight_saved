fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tz_name = args.get(1).map(String::as_str).unwrap_or_else(|| {
        println!("ERROR: Please provide a time zone to generate a transitions .ics for!");
        std::process::exit(1)
    });
    println!("{}", daylight_saved::generate_transitions_ics(tz_name).unwrap());
}
