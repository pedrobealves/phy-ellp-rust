use std::f64::consts::PI;
use crate::car::Movement;

#[derive(Clone, Copy, PartialEq)]

pub struct State {
    pub x: f64,
    pub v: f64,
    pub th: f64,
}

impl Default for State {
    fn default() -> Self {
        Self::from(0.0, 0.0, 0.0)
    }
}

impl State {
    pub fn from(x: f64, v: f64, th: f64) -> Self {
        State { x, v, th }
    }

    pub fn update(&mut self, a: f64, v0: f64, x0:f64 , dt: f64, movement: &Movement) {
        self.th = (dt * 100.0);
        match movement {
            Movement::MRU => {
                self.v = v0;
                self.x = x0 + v0*self.th;
            },
            Movement::MRUV => {
                self.v = v0 + a*self.th;
                self.x =  x0 + v0*self.th  + (0.5 * a * self.th.powi(2))as f64;
            },
        }
        //println!("v: {}", self.v);
        //println!("x: {}", self.x);
    }

}
