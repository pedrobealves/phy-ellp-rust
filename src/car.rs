#![allow(non_snake_case)]

use std::f64::consts::PI;

use macroquad::prelude::*;

use crate::{camera::CameraDynamics, state::State};
#[derive(PartialEq, Eq)]
pub enum Integrator {
    Euler,
    RungeKutta4,
}

impl Default for Integrator {
    fn default() -> Self {
        Self::RungeKutta4
    }
}

#[derive(PartialEq)]
pub struct Cart {
    pub ui_scale: f32,
    pub enable: bool,
    pub state: State,
    pub integrator: Integrator,
    pub steps: i32,
    pub R: f64,
    pub camera: CameraDynamics,
    g: f64,
    x: f64,
    x0: f64,
    v0: f64,
    v: f64,
    a: f64,
}

impl Default for Cart {
    fn default() -> Self {

        Cart {
            g: 9.80665,
            R: 0.1,
            state: State::default(),
            ui_scale: 0.3,
            steps: 1,
            enable: false,
            integrator: Integrator::default(),
            camera: CameraDynamics::default(),
            x: 0.0,
            x0: 0.0,
            v0: 0.0,
            v: 0.0,
            a: 0.0,
        }
    }
}

impl Cart {
    pub fn update(&mut self, dt: f64) {
        self.camera.update(self.state.x, self.state.v, dt);
        if (self.enable){
            let steps = self.steps;
            let dt = dt / steps as f64;
                if is_key_down(KeyCode::Left) {
                    self.a = self.a - 1.0;
                    print!("a: {}\n", self.a);
                } else if is_key_down(KeyCode::Right) {
                    self.a = self.a + 1.0;
                    print!("a: {}\n", self.a);
                }

                self.state.update((0.0,self.a,0.0,0.0), dt);
        }
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
        let x = (self.state.x - self.camera.y) as f32 * self.ui_scale;
        let R = self.R as f32 * self.ui_scale;
        let (c, s) = (
            (self.state.x / self.R).cos() as f32,
            (self.state.x / self.R).sin() as f32,
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
        // cart
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