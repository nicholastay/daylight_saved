fn fail_with_usage() -> ! {
    println!("Usage: ./daylight_saved <timezone> <years prior> <years future>");
    std::process::exit(1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tz_name = args.get(1).map(String::as_str).unwrap_or_else(|| fail_with_usage());
    let years_context_prior = args.get(2)
        .map(String::as_str).map(|x| str::parse::<i16>(x).unwrap()).unwrap_or_else(|| fail_with_usage());
    let years_context_future = args.get(3)
        .map(String::as_str).map(|x| str::parse::<i16>(x).unwrap()).unwrap_or_else(|| fail_with_usage());
    println!("{}", daylight_saved::generate_transitions_ics(tz_name, years_context_prior, years_context_future).unwrap());
}
