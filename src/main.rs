extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::thread_rng;
use rand::seq::SliceRandom;

mod color;
mod position;
mod tetromino;

use color::Color;
use tetromino::Tetromino;
use tetromino::shape::Shape;
use tetromino::shape::ATOM_SIZE;
use tetromino::shape::MARGIN_SIZE;


static WINDOW_WIDTH :u32 = 10*ATOM_SIZE + MARGIN_SIZE;
static WINDOW_HEIGHT :u32 = 22*ATOM_SIZE + MARGIN_SIZE;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = thread_rng();

    canvas.set_draw_color(Color::WHITE.rgb());
    canvas.clear();
    canvas.present();

    let shapes = [ Shape::I, Shape::J, Shape::L, Shape::O, Shape::S, Shape::T, Shape::Z ];

    let (x, y) :(i32, i32) = (MARGIN_SIZE as i32, MARGIN_SIZE as i32);
    let shape = shapes.choose(&mut rng).expect("Cannot get something out of `shapes`.");
    let mut current_tetromino :Tetromino = Tetromino::new(x, y, shape.clone());

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } | Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    current_tetromino.move_right();
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } | Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    current_tetromino.move_left();
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::WHITE.rgb());
        canvas.clear();

        current_tetromino.move_down();
        current_tetromino.draw_on(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
