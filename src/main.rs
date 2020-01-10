extern crate sdl2;
extern crate libm;

mod orbit;

use libm::atan2f;

use orbit::co_class::CelestialObject;
use orbit::orbit::orbit;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread::sleep;
use std::path::Path;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_sub = sdl.video().unwrap();

    let window = video_sub.window("Orbital simulation", 1400, 950).build().unwrap();
    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let mut texture_creator = canvas.texture_creator();
    let mut event_handler = sdl.event_pump().unwrap();
    let mut run = 0;

    let mut earth = CelestialObject {pos_x: 600.0, pos_y: 600.0, mass: 20.0};
    let mut sun = CelestialObject { pos_x: 700.0, pos_y: 475.0, mass: 80.0};

    let mut Sun = Rect::new(sun.pos_x as i32, sun.pos_y as i32, 50, 50);
    let mut Earth = Rect::new(earth.pos_x as i32, earth.pos_y as i32, 10, 10);

    canvas.set_draw_color(Color::RGBA(16, 32, 64, 255));

    'run: loop {
        for event in event_handler.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'run;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    run += 1;
                    if run > 1 {
                        run = 0;
                    }

                    println!("{}", &run);
                },
                _ => {}
            }
        }

        // Logic
        if run == 1 {
            orbit(&mut earth, &mut sun);
        }

        &Earth.set_x(earth.pos_x as i32);
        &Earth.set_y(earth.pos_y as i32);


        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 62, 28));
        canvas.fill_rect(Sun);
        canvas.set_draw_color(Color::RGB(16, 32, 64));
        canvas.fill_rect(Earth);

        canvas.present();
        sleep(Duration::new(1 / 60, 0));
    }

}