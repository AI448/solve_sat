#[derive(Clone, Debug)]
pub struct PLBDWatcher {
    long_term_average: Average,
    short_term_average: Current,
}

impl PLBDWatcher {
    pub fn new(long_term_time_constant: f64, short_term_time_constant: f64) -> Self {
        Self {
            long_term_average: Average::new(long_term_time_constant),
            short_term_average: Current::new(short_term_time_constant.ceil() as usize),
        }
    }

    pub fn add(&mut self, plbd: u32) {
        self.long_term_average.add(plbd as f64);
        self.short_term_average.add(plbd as f64);
    }

    // pub fn count(&self) -> usize {
    //     self.short_term_average.count
    // }

    pub fn long_term_average(&self) -> f64 {
        self.long_term_average.value()
    }

    // pub fn short_term_average(&self) -> f64 {
    //     self.short_term_average.value()
    // }

    pub fn ratio(&self) -> f64 {
        if self.long_term_average.value() != 0.0 {
            self.short_term_average.value() / self.long_term_average.value()
        } else {
            if self.short_term_average.value() == 0.0 { 0.0 } else { f64::INFINITY }
        }
    }

    pub fn clear_short_term_average(&mut self) {
        self.short_term_average.clear()
    }
}

#[derive(Clone, Debug)]
struct Average {
    time_constant: f64,
    count: f64,
    mean: f64,
}

impl Average {
    fn new(time_constant: f64) -> Self {
        assert!(time_constant > 0.0);
        Self { time_constant: time_constant, count: 0.0, mean: 0.0 }
    }

    fn add(&mut self, value: f64) {
        self.count += 1.0;
        let t = self.time_constant.min(self.count);
        self.mean = ((t - 1.0) * self.mean + value) / t;
    }

    fn value(&self) -> f64 {
        if self.count != 0.0 { self.mean } else { f64::NAN }
    }
}

#[derive(Clone, Debug)]
struct Current {
    ring_buffer: Vec<f64>,
    first: usize,
    count: usize,
    sum: f64,
}

impl Current {
    fn new(time_constant: usize) -> Self {
        assert!(time_constant > 0);
        Self {
            ring_buffer: Vec::from_iter(std::iter::repeat(f64::NAN).take(time_constant)),
            first: 0,
            count: 0,
            sum: 0.0,
        }
    }

    fn add(&mut self, value: f64) {
        if self.count == self.ring_buffer.len() {
            self.sum -= self.ring_buffer[self.first];
            self.first = (self.first + 1) % self.ring_buffer.len();
        } else {
            self.count += 1;
        }
        let last = (self.first + self.count - 1) % self.ring_buffer.len();
        self.ring_buffer[last] = value;
        self.sum += value;
    }

    fn value(&self) -> f64 {
        self.sum / self.ring_buffer.len() as f64
    }

    fn clear(&mut self) {
        self.first = 0;
        self.count = 0;
        self.sum = 0.0;
    }
}
