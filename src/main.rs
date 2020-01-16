extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod color;
mod position;
mod tetromino;

use color::Color;
use tetromino::Tetromino;
use tetromino::shape::Shape;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::WHITE.rgb());
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Examples of tetrominos on the screen
    let shapes = [ Shape::I, Shape::J, Shape::L, Shape::O, Shape::S, Shape::T, Shape::Z ];
    let tetrominos = shapes.iter().enumerate()
        .map(|(i, shape)| (10 + (i as i32)*100, 10, shape))
        .map(|(x, y, shape)| Tetromino::new(x, y, shape.clone()));

    for tetromino in tetrominos {
        tetromino.draw_on(&mut canvas);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
