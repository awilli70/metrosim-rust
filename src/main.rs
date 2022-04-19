use std::env;
use std::process;

use metrosim::{state::State, Config};
fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Error parsing command line args: {}", e);
        process::exit(1)
    });
    let mut state = State::new()
        .initialize_stations(&cfg.stations_file)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing command line args: {}", e);
            process::exit(1)
        });
    let _res = metrosim::run_simulation(&cfg, &mut state).unwrap();
}
