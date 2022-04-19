use std::fmt;

use crate::passenger::Passenger;
#[derive(Clone)]
pub struct PassengerQueue {
    pub passenger_list: Vec<Passenger>,
}

impl PassengerQueue {
    pub fn new() -> PassengerQueue {
        PassengerQueue {
            passenger_list: vec![],
        }
    }

    pub fn front(&self) -> Passenger {
        if let Some(p) = self.passenger_list.get(0) {
            return p.clone();
        } else {
            panic!("Took from front, when queue was empty");
        }
    }
    pub fn dequeue(&mut self) {
        self.passenger_list = self.passenger_list.drain(1..).collect();
    }
    pub fn enqueue(&mut self, p: Passenger) {
        self.passenger_list.push(p);
    }
    pub fn size(&self) -> u32 {
        self.passenger_list.len().try_into().unwrap()
    }
    pub fn print(&self) {
        print!("{}", self);
    }
}

impl fmt::Display for PassengerQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for p in &self.passenger_list {
            s.push_str(&format!("{}", p));
        }
        return write!(f, "{}", s);
    }
}
