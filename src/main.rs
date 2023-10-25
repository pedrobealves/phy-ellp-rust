use crate::{theme::setup_theme, ui::Graph};
use car::Cart;
use egui::{pos2, Color32};
use macroquad::prelude::*;
use ui::{draw_blue_grid, draw_speedometer, draw_ui, draw_vingette};
use crate::car::Movement;
use crate::report::Report;

mod camera;
mod car;
mod state;
mod theme;
mod ui;
mod report;
mod timer;
fn window_conf() -> Conf {
    Conf {
        window_title: "Cart".to_string(),
        fullscreen: false,
        // window_resizable: false,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let grid = 0.15;
    let w_init = 1280.;
    let mut cart = Cart::default();
    let report:Report = Report::default();
    let vingette = Texture2D::from_file_with_format(include_bytes!("../vingette.png"), None);
    let font = load_ttf_font_from_bytes(include_bytes!("../Ubuntu-Regular.ttf")).unwrap();
    setup_theme();
    let mut report = Report::default();
    let mut forceplt = Graph::new(
        &["Movimentos", "Velocidade", "Aceleração"],
        pos2((0.5 - 2. * grid) * w_init, 0.),
        egui::vec2(1.5, 1.) * grid * w_init,
        Some([Color32::WHITE, Color32::LIGHT_GREEN].to_vec()),
    );
    let mut forceplt1 = Graph::new(
        &["POS", "Posição"],
        pos2((0.5 + 0.5 * grid) * w_init, 0.),
        egui::vec2(1.5, 1.) * grid * w_init,
        Some([Color32::LIGHT_RED].to_vec()),
    );
    next_frame().await;
    let back_color = Color::from_hex(0xf1ba0a);

    loop {
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if get_time() > 0. {
            cart.update(get_frame_time() as f64);
        }

        if (cart.enable){
            forceplt.update([cart.get_last_state().unwrap().v, cart.a]
                .to_vec(), cart.get_last_state().unwrap().th);
            forceplt1.update([cart.get_last_state().unwrap().x].to_vec(), cart.get_last_state().unwrap().th);
        }


        clear_background(back_color);
        draw_blue_grid(grid, Color::from_hex(0xf6de26), 0.001, 3, 0.003);

        cart.display(back_color, WHITE, 0.006, 6. * grid, 3. * grid);
        draw_speedometer(
            &format!(
                "Velocidade ({}) {:.2}",
                if cart.state.v.is_sign_negative() {
                    "-"
                } else {
                    "+"
                },
                cart.state.v.abs()
            ),
            vec2(0., screen_height() / screen_width() - 0.75 * grid),
            0.08,
            cart.state.v as f32,
            100.,
            0.8,
            font,
            14.,
            true,
        );
        if cart.movement == Movement::MRUV {
            draw_speedometer(
                &format!(
                    "Aceleração ({}) {:.2}",
                    if cart.a.is_sign_negative() {
                        "-"
                    } else {
                        "+"
                    },
                    cart.a.abs()
                ),
                vec2(0., screen_height() / screen_width() - 1.75 * grid),
                0.08,
                cart.a as f32,
                20.,
                0.8,
                font,
                14.,
                true,
            );
        }
        draw_ui(w_init, grid, &mut cart, &mut forceplt, &mut forceplt1, &mut report);
        draw_vingette(vingette);
        next_frame().await;
    }
}