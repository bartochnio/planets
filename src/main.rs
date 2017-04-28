extern crate sdl2;

mod engine;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::*;
use std::time::Duration;
use std::boxed::Box;
use engine::gameobjects::*;



fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().unwrap();


    let mut renderer = window.renderer()
        .accelerated().build().unwrap();

    renderer.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));


    let mut timer = sdl_context.timer().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    /*
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/animate.bmp")).unwrap();
    let texture = renderer.create_texture_from_surface(&temp_surface).unwrap();
    */
    let obj = ObjectData {
        start_position : Point::new(320,240),
        draw_rect : Rect::new(0, 0, 128, 82),
        texure_name : "animate.bmp".to_string(),
    };

    let game_object;
    {
        let texture_factory = TextureFactory::new(&renderer);
        game_object = ObjectsFactory::create_object(&obj, &texture_factory);
    }


    // Main shitty loop
    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        let ticks = timer.ticks();

        renderer.clear();
        game_object.DrawToRenderer(&mut renderer);
        renderer.present();

        std::thread::sleep(Duration::from_millis(100));
    }
}





