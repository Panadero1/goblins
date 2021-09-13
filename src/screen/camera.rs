use crate::world::space::GamePos;

pub struct Camera {
    pos: GamePos,
}
impl Camera {
    pub fn new(pos: (f32, f32)) -> Camera {
        Camera {
            pos: pos.into()
        }
    }
    pub fn moove(&mut self, change_pos: (f32, f32)) {
        self.pos += (change_pos.0, change_pos.1).into();
    }
    pub fn game_to_pix(&self, game_pos: GamePos) -> (f32, f32) {
        todo!();
    }
}