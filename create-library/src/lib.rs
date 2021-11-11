
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64
}

impl Fern {
    /// simulate fern growing for one day
    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// run a fern simulation for some number of days
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
