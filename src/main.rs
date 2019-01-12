extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct Pos {
    x: i32,
    y: i32
}
struct World {
    tick: u32,
    cursor_pos: Pos
}
struct RenderCanvas {
    canvas: Canvas<Window>
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut render_canvas = RenderCanvas::init(&sdl_context);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut pos = Pos { x: 0, y: 0 };
    let mut world = World { tick: 0, cursor_pos: Pos { x: 0, y: 0} };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
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

fn tick(pos: &Pos, world: &mut World) {
    world.cursor_pos.x = pos.x;
    world.cursor_pos.y = pos.y;
    world.tick = world.tick + 1;
}

impl RenderCanvas {
    fn init(sdl_context: &sdl2::Sdl) -> RenderCanvas {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        return RenderCanvas { canvas };
    }

    fn render(&mut self, world: &World) {
        let ww = 30;
        let hh = 40;
        let rect = Rect::new(
            world.cursor_pos.x - (ww / 2) as i32,
            world.cursor_pos.y - (hh / 2) as i32, ww, hh
        );

        let r: u8 = (world.tick % 255) as u8;
        let b: u8 = (255 - world.tick % 255) as u8;

        self.canvas.set_draw_color(Color::RGB(r, 64, b));
        self.canvas.fill_rect(rect).unwrap();
        self.canvas.present();
    }
}