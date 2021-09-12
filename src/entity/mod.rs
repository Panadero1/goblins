use speedy2d::Graphics2D;

use crate::utility::animation::AnimationSelectError;

pub mod goblin;
pub mod player;

pub trait Entity {
    fn draw(&mut self, graphics: &mut Graphics2D);
    fn moove(&mut self, change_pos: (f32, f32));
    fn set_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError>;
    fn intercept_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError>;
    fn remove_anim(&mut self);
}