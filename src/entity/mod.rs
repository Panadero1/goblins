use speedy2d::Graphics2D;

pub mod goblin;
pub mod player;

pub trait Entity {
    fn draw(&self, graphics: &mut Graphics2D);
    fn moove(&mut self, change_pos: (f32, f32));
}