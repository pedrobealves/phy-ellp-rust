use std::f64::consts::PI;
use crate::car::Movement;

#[derive(Clone, Copy, PartialEq)]

pub struct State {
    pub x: f64, //Posição
    pub v: f64, //Velocidade
    pub th: f64, //Tempo
}

impl Default for State {
    fn default() -> Self {
        Self::from(0.0, 0.0, 0.0)
    }
}

impl State {
    // Recebe uma posição, velocidade e tempo e retorna um estado
    pub fn from(x: f64, v: f64, th: f64) -> Self {
        State { x, v, th }
    }

    // Atualiza o estado
    pub fn update(&mut self, a: f64, v0: f64, x0:f64 , dt: f64, movement: &Movement) {
        self.th = (dt * 100.0);
        // Verifica o tipo de movimento
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
