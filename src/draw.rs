use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOC_SIZE: f64 = 25.0;

pub fn to_coord(game_record: i32) -> f64 {
    (game_record as f64) * BLOC_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOC_SIZE, BLOC_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOC_SIZE * (width as f64),
            BLOC_SIZE * (height as f64),
        ],
        con.transform,
        g,
    )
}
