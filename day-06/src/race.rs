#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    /// Return the number of ways to win the race
    pub fn get_n_ways(&self) -> u64 {
        let t = self.time as f64;
        let r = self.distance as f64;
        let disc = (t.powf(2.0) - (4.0 * r)).sqrt();
        let min = (-t + disc) / -2.0;
        let max = (-t - disc) / -2.0;
        max.ceil() as u64 - min.floor() as u64 - 1
    }
}
