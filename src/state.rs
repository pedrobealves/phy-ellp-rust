use std::f64::consts::PI;

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

    pub fn update(&mut self, a: f64, dt: f64) {
        self.th = dt;
        self.v = a*dt;
        self.x += self.v * dt + 0.5 * a * dt.powi(2) as f64;
        println!("x: {}", self.x);
        println!("y: {}", self.x);
    }

}
