extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod render;
mod world;

use render::RenderCanvas;
use world::World;
use world::Pos;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut render_canvas = RenderCanvas::init(&sdl_context);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut pos = Pos { x: 0, y: 0 };
    let mut world = World { tick: 0, cursor_pos: Pos { x: 0, y: 0 } };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::MouseMotion { x, y, .. } => {
                    pos.x = x;
                    pos.y = y;
                }
                _ => {}
            };
        };

        tick(&pos, &mut world);

        render_canvas.render(&world);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}

fn tick(pos: &Pos, world: &mut world::World) {
    world.cursor_pos.x = pos.x;
    world.cursor_pos.y = pos.y;
    world.tick = world.tick + 1;
}
