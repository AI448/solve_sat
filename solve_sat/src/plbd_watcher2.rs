use core::f64;

use statrs::function::erf::erf;

#[derive(Clone, Debug)]
pub struct PLBDWatcher2 {
    log_x1: Average,
    log_x2: Average,
}

impl PLBDWatcher2 {
    pub fn new(time_constant: f64) -> Self {
        Self { log_x1: Average::new(time_constant), log_x2: Average::new(time_constant) }
    }

    pub fn add(&mut self, plbd: u32) {
        let log_x = f64::ln(plbd as f64);
        self.log_x1.add(log_x);
        self.log_x2.add(log_x * log_x);
    }

    pub fn log_mean(&self) -> f64 {
        self.log_x1.value()
    }

    pub fn log_var(&self) -> f64 {
        f64::max(0.0, self.log_x2.value() - self.log_x1.value().powf(2.0))
    }

    pub fn cfd(&self, x: f64) -> f64 {
        let mean = self.log_mean();
        let var = self.log_var();
        let cfd = 0.5 * (1.0 + erf((f64::ln(x) - mean) / f64::sqrt(2.0 * var)));
        return cfd;
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
