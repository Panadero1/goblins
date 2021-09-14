use std::{cmp::Ordering, collections::HashMap};

use speedy2d::{
    color::Color,
    image::{ImageDataType, ImageFileFormat, ImageHandle},
    shape::Rectangle,
};

use crate::{
    screen::camera::Camera,
    utility::animation::{Animation, AnimationSelectError},
    world::space::GamePos,
};

use super::Entity;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

pub struct Tile<'a> {
    pos: GamePos,
    anim: Animation<'a>,
    game_size: (f32, f32),
}

impl<'a> Entity for Tile<'a> {
    fn draw(&mut self, graphics: &mut speedy2d::Graphics2D, camera: &Camera) {
        self.anim.draw(
            graphics,
            Rectangle::from_tuples(
                camera.game_to_pix(self.pos),
                camera.game_to_pix(
                    (self.pos.x + self.game_size.0, self.pos.y + self.game_size.1).into(),
                ),
            ),
            Color::WHITE,
        );
    }
    fn moove(&mut self, change_pos: (f32, f32)) {
        self.pos = (self.pos.x + change_pos.0, self.pos.y + change_pos.1).into();
    }
    fn set_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        Ok(())
    }
    fn intercept_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        Ok(())
    }
    fn remove_anim(&mut self) {}
    fn accelerate(&mut self, vector: GamePos) {}
    fn get_pos(&self) -> GamePos {
        self.pos
    }
}

impl<'a> Tile<'a> {
    pub fn new(src: ImageHandle, display: (u16, u16), pos: (f32, f32)) -> Tile<'a> {
        let anim = Animation::new(src, (5, 5), HashMap::new(), display, 100);
        Tile {
            pos: pos.into(),
            anim,
            game_size: (5.0, 5.0),
        }
    }
}
