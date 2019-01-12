use sdl2::render::Canvas;
use sdl2::video::Window;

use world::World;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct RenderCanvas {
    canvas: Canvas<Window>
}
impl RenderCanvas {
    pub fn init(sdl_context: &sdl2::Sdl) -> RenderCanvas {
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

    pub fn render(&mut self, world: &World) {
        let ww = 30;
        let hh = 40;
        let cursor_pos = world.get_cursor_pos();

        let rect = Rect::new(
            cursor_pos.x - (ww / 2) as i32,
            cursor_pos.y - (hh / 2) as i32, ww, hh
        );

        let tick = world.current_tick();
        let r: u8 = (tick % 255) as u8;
        let b: u8 = (255 - r) as u8;

        self.canvas.set_draw_color(Color::RGB(r, 64, b));
        self.canvas.fill_rect(rect).unwrap();
        self.canvas.present();
    }
}