use macroquad::prelude::get_time;

#[derive(PartialEq)]
pub struct Timer {
    start_time: Option<f64>,
    duration: f64,
    paused: bool,
    pause_time: f64,
}

impl Timer {
    pub fn new(duration: f64) -> Self {
        Timer {
            start_time: None,
            duration,
            paused: false,
            pause_time: 0.0,
        }
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(get_time());
        }
    }

    pub fn elapsed(&self) -> f64 {
        match self.start_time {
            Some(start_time) => {
                if self.paused {
                    self.pause_time - start_time
                } else {
                    get_time() - start_time
                }
            }
            None => 0.0,
        }
    }

    pub fn pause(&mut self) {
        if !self.paused && self.start_time.is_some() {
            self.pause_time = get_time();
            self.paused = true;
        }
    }

    pub fn resume(&mut self) {
        if self.paused {
            let pause_duration = get_time() - self.pause_time;
            self.start_time = Some(self.start_time.unwrap() + pause_duration);
            self.paused = false;
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.start_time {
            Some(start_time) => self.elapsed() >= self.duration + start_time,
            None => false,
        }
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.paused = false;
        self.pause_time = 0.0;
    }
}