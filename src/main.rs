extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod color;
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

        // Shape: I
        let tetromino_i = Tetromino::new(10, 10, Shape::I);
        tetromino_i.draw_on(&mut canvas);

        // Shape: J
        let tetromino_j = Tetromino::new(100, 10, Shape::J);
        tetromino_j.draw_on(&mut canvas);

        // Shape: L
        let tetromino_l = Tetromino::new(200, 10, Shape::L);
        tetromino_l.draw_on(&mut canvas);

        // Shape: O
        let tetromino_o = Tetromino::new(300, 10, Shape::O);
        tetromino_o.draw_on(&mut canvas);

        // Shape: S
        let tetromino_s = Tetromino::new(400, 10, Shape::S);
        tetromino_s.draw_on(&mut canvas);

        // Shape: T
        let tetromino_t = Tetromino::new(500, 10, Shape::T);
        tetromino_t.draw_on(&mut canvas);

        // Shape: Z
        let tetromino_z = Tetromino::new(600, 10, Shape::Z);
        tetromino_z.draw_on(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
