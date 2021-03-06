pub struct Pos {
    pub x: i32,
    pub y: i32
}
pub struct World {
    tick: u32,
    cursor_pos: Pos
}
impl World {
    pub fn next_tick(&mut self) {
        self.tick = self.tick + 1;
    }

    pub fn current_tick(&self) -> u32 {
        return self.tick;
    }

    pub fn write_cursor_pos(&mut self, pos: &Pos) {
        self.cursor_pos.x = pos.x;
        self.cursor_pos.y = pos.y;
    }

    pub fn get_cursor_pos(&self) -> &Pos {
        return &self.cursor_pos;
    }

    pub fn new() -> World {
        return World { tick: 0, cursor_pos: Pos { x: 0, y: 0 } }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_update_the_tick() {
        let mut subject = World::new();

        // Exercise
        subject.next_tick();

        // Verify
        assert_eq!(subject.current_tick(), 1)
    }

    #[test]
    fn it_should_write_new_cursor_position() {
        let mut subject = World::new();

        // Exercise
        subject.write_cursor_pos(&Pos { x: 100, y: 200 });

        // Verify
        assert_eq!(subject.get_cursor_pos().x, 100);
        assert_eq!(subject.get_cursor_pos().y, 200);
    }
}