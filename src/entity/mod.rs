use speedy2d::Graphics2D;

pub mod goblin;
pub mod player;

pub trait Entity {
    fn draw(&mut self, graphics: &mut Graphics2D);
    fn moove(&mut self, change_pos: (f32, f32));
    fn set_anim(&mut self, anim_name: &str) -> Result<(), ()>;
    fn remove_anim(&mut self);
}