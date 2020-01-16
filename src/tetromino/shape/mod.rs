use sdl2::render::Canvas;
use sdl2::pixels::Color as SDL2Color;
use sdl2::rect::Rect;
use sdl2::video::Window;

use crate::tetromino::Position;

static ATOM_SIZE :i32 = 30;

#[derive(Clone)]
pub enum Shape {
    I, J, L, O, S, T, Z
}


fn draw_I_on(canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
    let atom_size = 30;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.x, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + ATOM_SIZE + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + 2*(ATOM_SIZE + 1), atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + 3*(ATOM_SIZE + 1), atom_size, atom_size));
}

fn draw_J_on(canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
    let atom_size = 30;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + (atom_size as i32) + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + 2*((atom_size as i32) + 1), atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + 2*(ATOM_SIZE + 1), atom_size, atom_size));
}

fn draw_L_on(canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
    let atom_size = 30;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.x, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + ATOM_SIZE + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + 2*(ATOM_SIZE + 1), atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + 2*((atom_size as i32) + 1), atom_size, atom_size));
}

fn draw_O_on(canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
    let atom_size = 30;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.x, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + ATOM_SIZE + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + (atom_size as i32) + 1, atom_size, atom_size));
}

fn draw_S_on(canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
    let atom_size = 30;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.x, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + ATOM_SIZE + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + (atom_size as i32) + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + 2*((atom_size as i32) + 1), atom_size, atom_size));
}

fn draw_T_on(canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
    let atom_size = 30;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + ATOM_SIZE + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + (atom_size as i32) + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + 2 * (ATOM_SIZE + 1), position.y + (atom_size as i32) + 1, atom_size, atom_size));
}

fn draw_Z_on(canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
    let atom_size = 30;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + ATOM_SIZE + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x + ATOM_SIZE + 1, position.y + (atom_size as i32) + 1, atom_size, atom_size));
    canvas.fill_rect(Rect::new(position.x, position.y + 2*(ATOM_SIZE + 1), atom_size, atom_size));
}

impl Shape {
    pub fn draw_on(&self, canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
        match self {
            Shape::I => draw_I_on(canvas, position, color),
            Shape::J => draw_J_on(canvas, position, color),
            Shape::L => draw_L_on(canvas, position, color),
            Shape::O => draw_O_on(canvas, position, color),
            Shape::S => draw_S_on(canvas, position, color),
            Shape::T => draw_T_on(canvas, position, color),
            Shape::Z => draw_Z_on(canvas, position, color),
        }
    }
}

