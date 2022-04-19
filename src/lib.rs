use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    process,
};

use passenger_queue::PassengerQueue;

mod passenger;
mod passenger_queue;
pub mod state;

pub struct Config {
    pub stations_file: String,
    pub output_file: String,
    pub commands_file: String,
    pub commands_file_incl: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let stations_file = match args.next() {
            Some(s) => s,
            None => return Err("No stations file provided"),
        };
        let output_file = match args.next() {
            Some(s) => s,
            None => return Err("No outfile provided"),
        };
        let commands_file;
        let commands_file_incl;

        match args.next() {
            Some(s) => {
                commands_file = s;
                commands_file_incl = true;
            }
            None => {
                commands_file = String::from("");
                commands_file_incl = false;
            }
        }

        return Ok(Config {
            stations_file,
            output_file,
            commands_file,
            commands_file_incl,
        });
    }
}

fn parse_and_execute(cfg: &Config, state: &mut state::State, command: String) -> bool {
    let mut words = command.trim().split(" ");
    if let Some(s) = words.next() {
        match s {
            "p" => {
                add_passenger(state, words.collect());
                return true;
            }
            "m" => {
                if let Some(c) = words.next() {
                    match c {
                        "m" => {
                            move_train(cfg, state);
                            return true;
                        }
                        "f" => {
                            end_simulation();
                            return false;
                        }
                        _ => panic!("Malformed Command"),
                    }
                }
            }
            _ => panic!("Malformed Command"),
        }
    }
    return false;
}

fn add_passenger(state: &mut state::State, stations: Vec<&str>) {
    if stations.len() < 2 {
        panic!("Malformed Add Passenger Command");
    }
    let start: usize = stations[0].parse().expect("Malformed Enter Station Number");
    let end: usize = stations[1].parse().expect("Malformed Exit Station Number");
    state.add_passenger(start, end);
}

fn move_train(cfg: &Config, state: &mut state::State) {
    let current_station: &mut PassengerQueue = state
        .stations
        .get_mut(state.train_loc)
        .expect("Train isn't at a station?");
    while current_station.size() > 0 {
        let p = current_station.front();
        let train_car = state
            .train
            .get_mut(p.departure)
            .expect("Train is missing this car");
        train_car.enqueue(p);
        current_station.dequeue();
    }
    state.train_loc = (state.train_loc + 1) % state.num_stations;
    let to_disembark: &mut PassengerQueue = state
        .train
        .get_mut(state.train_loc)
        .expect("Moved off the face of the earth");
    let mut of: fs::File = fs::File::create(&cfg.output_file).expect("Failed to open outfile");
    while to_disembark.size() > 0 {
        let p = to_disembark.front();
        of.write(
            &(format!(
                "Passenger {} left train at station {}",
                p.id,
                state.station_map.get(&p.departure).unwrap()
            ))
            .as_bytes(),
        )
        .expect("Could not write to file");
        to_disembark.dequeue();
    }
}

fn end_simulation() {
    println!("Thank you for playing MetroSim. Have a nice day!");
    process::exit(0);
}

pub fn run_simulation(cfg: &Config, state: &mut state::State) -> Result<(), Box<dyn Error>> {
    if cfg.commands_file_incl {
        let commands = fs::read_to_string(&cfg.commands_file)?;
        commands.lines().for_each(|cmd| {
            parse_and_execute(cfg, state, cmd.to_string());
            state.print_state();
        });
    } else {
        loop {
            let mut commandbuf: String = String::new();
            io::stdin()
                .read_line(&mut commandbuf)
                .expect("Failed to read command");
            parse_and_execute(cfg, state, commandbuf);
            state.print_state();
        }
    }
    return Ok(());
}
