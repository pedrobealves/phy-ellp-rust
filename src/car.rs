#![allow(non_snake_case)]

use std::collections::LinkedList;

use macroquad::prelude::*;

use crate::{camera::CameraDynamics, state::State, timer::Timer};

#[derive(PartialEq, Eq)]
pub enum Movement {
    MRU,
    MRUV,
}

impl Default for Movement {
    fn default() -> Self {
        Self::MRUV
    }
}

#[derive(PartialEq)]
pub struct Car {
    pub ui_scale: f32, //Escala de tamanho da UI
    pub enable: bool, //Habilita a simulação
    pub state: State, //Estado atual do carrinho
    pub movement: Movement, //Tipo de movimento
    pub steps: i32, //Quantidade de passos por frame
    pub R: f64, //Raio da roda
    pub camera: CameraDynamics, //Dinâmica da câmera
    pub a: f64, //Aceleração
    pub x0: f64, //Posição inicial
    pub v0: f64, //Velocidade inicial
    state_history: LinkedList<State>, // Histórico de estados
    timer: Timer, //Timer
}

impl Default for Car {
    fn default() -> Self {
        Car {
            R: 0.1,
            state: State::default(),
            ui_scale: 0.3,
            steps: 1,
            enable: false,
            movement: Movement::default(),
            camera: CameraDynamics::default(),
            state_history: LinkedList::new(),
            timer: Timer::new(0.0),
            x0: 0.0,
            v0: 0.0,
            a: 2.0,
        }
    }
}

impl Car {
    pub fn update(&mut self, dt: f64) {
        self.camera.update(self.get_position() , self.state.v, dt);
        if self.enable {
            self.timer.start();
            let t = self.timer.elapsed() / 100.0;
            if let Movement::MRU = self.movement {
                self.a = 0.0;
            }
            self.state.update(self.a , self.v0, self.x0, t, &self.movement);
            self.state_history.push_back(self.state.clone());
            let steps = self.steps;
            let dt = dt / steps as f64;
            if is_key_down(KeyCode::Left) {
                self.a= self.a - 0.1;
            } else if is_key_down(KeyCode::Right) {
                self.a= self.a + 0.1;
            }
        }
    }

    pub fn reset_timer(&mut self) {
        self.timer.reset();
    }

    pub fn get_position(&self) -> f64 {
        self.state.x / 100.0
    }

    pub fn get_last_state(&self) -> Option<&State> {
        self.state_history.iter().last()
    }

    pub fn enable(&mut self){
        match self.enable {
            true => self.timer.resume(),
            false => self.timer.pause(),
        }
    }

    pub fn get_state_history(&self) -> &LinkedList<State> {
        &self.state_history
    }

    pub fn reset_state_history(&mut self) {
        self.state_history.clear();
    }

    pub fn display(
        &self,
        back_color: Color,
        color: Color,
        thickness: f32,
        length: f32,
        depth: f32,
    ) {
        draw_line(-length, -depth, length, -depth, thickness, color);
        let x = (self.get_position() - self.camera.y) as f32 * self.ui_scale;
        let R = self.R as f32 * self.ui_scale;
        let (c, s) = (
            (self.get_position() / self.R).cos() as f32,
            (self.get_position() / self.R).sin() as f32,
        );

        let ticks = (9. / self.ui_scale) as i32;
        let gap = 2. / ticks as f32;
        let offset = (self.camera.y as f32 * self.ui_scale) % gap;
        for i in 0..ticks + 2 {
            draw_line(
                (-offset + gap * i as f32 - 1.) * length,
                -depth - 0.002,
                (-offset + gap * i as f32 - 1.) * length - 0.1 * self.ui_scale,
                -depth - 0.1 * self.ui_scale,
                thickness,
                color,
            );
        }
        draw_rectangle(
            -1.,
            -depth - 0.001,
            1. - length - 0.003,
            -0.11 * self.ui_scale,
            back_color,
        );
        draw_rectangle(
            length + 0.003,
            -depth - 0.001,
            1. - length - 0.003,
            -0.11 * self.ui_scale,
            back_color,
        );

        let (w, h) = (R * 10., R * 3.5);
        // car
        draw_rectangle_lines(x - 0.5 * w, -depth + 2. * R, w, h, thickness * 2., color);

        // wheels
        draw_circle_lines(x - 0.30 * w, -depth + R, R, thickness, color);
        draw_circle_lines(x + 0.30 * w, -depth + R, R, thickness, color);
        draw_line(
            x - 0.30 * w,
            -depth + R,
            x - 0.30 * w - R * c,
            -depth + R + R * s,
            thickness,
            color,
        );
        draw_line(
            x + 0.30 * w,
            -depth + R,
            x + 0.30 * w - R * c,
            -depth + R + R * s,
            thickness,
            color,
        );

        let (c, s) = ((self.state.th).cos() as f32, (self.state.th).sin() as f32);

    }
}
