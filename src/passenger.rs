use std::fmt;

#[derive(Clone)]
pub struct Passenger {
    pub id: u32,
    pub arrival: usize,
    pub departure: usize,
}

impl fmt::Display for Passenger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}->{}]", self.id, self.arrival, self.departure)
    }
}
