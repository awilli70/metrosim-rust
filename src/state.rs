use std::{collections::HashMap, error::Error, fs};

use crate::passenger::Passenger;
use crate::passenger_queue::PassengerQueue;

pub struct State {
    pub station_map: HashMap<usize, String>,
    pub num_stations: usize,
    curr_passenger_num: u32,
    pub train: Vec<PassengerQueue>,
    pub train_loc: usize,
    pub stations: Vec<PassengerQueue>,
}

struct Counter {
    val: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { val: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_count = self.val;
        self.val += 1;
        return Some(curr_count);
    }
}

impl State {
    pub fn new() -> State {
        State {
            station_map: HashMap::new(),
            num_stations: 0,
            curr_passenger_num: 0,
            train: vec![],
            train_loc: 0,
            stations: vec![],
        }
    }

    pub fn initialize_stations(&mut self, stationfile: &str) -> Result<State, Box<dyn Error>> {
        let mut state: State = State::new();
        let counter: Counter = Counter::new();
        let station_names = fs::read_to_string(stationfile)?;

        state.station_map = counter
            .zip(station_names.lines().map(|s| s.to_string()))
            .into_iter()
            .collect();

        state.num_stations = state.station_map.len();

        state.train = vec![PassengerQueue::new(); state.num_stations];
        state.stations = vec![PassengerQueue::new(); state.num_stations];

        println!("{:?}", state.station_map);
        println!("{} stations total", state.num_stations);

        return Ok(state);
    }

    pub fn add_passenger(&mut self, start: usize, end: usize) {
        let stationqueue: &mut PassengerQueue = self
            .stations
            .get_mut(start)
            .expect("Station doesn't exist?");
        let p: Passenger = Passenger {
            id: self.curr_passenger_num,
            arrival: start,
            departure: end,
        };
        stationqueue.enqueue(p);
        self.curr_passenger_num += 1;
    }

    pub fn print_state(&self) {
        let mut print_str: String = String::new();
        print_str.push_str("Passengers on the train: {");
        for traincar in &self.train {
            for passenger in &traincar.passenger_list {
                print_str.push_str(&format!("{}", passenger));
            }
        }
        print_str.push_str("}\n");
        for i in 0..self.num_stations {
            if i == self.train_loc {
                print_str.push_str("TRAIN: ");
            } else {
                print_str.push_str("       ");
            }
            print_str.push_str(&format!(
                "[{}] {} {{",
                i,
                self.station_map.get(&i).expect("Station has no name?")
            ));
            let curr_station = self.stations.get(i).expect("No station?");
            print_str.push_str(&format!("{}", curr_station));
            print_str.push_str("}\n");
        }
        print!("{}", print_str);
    }
}
