pub struct Camera {
    pos: (f32, f32),
}
impl Camera {
    pub fn new(pos: (f32, f32)) -> Camera {
        Camera {
            pos
        }
    }
    pub fn moove(&mut self, change_pos: (f32, f32)) {
        self.pos.0 += change_pos.0;
        self.pos.1 += change_pos.1;
    }
}