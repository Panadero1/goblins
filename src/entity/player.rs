use std::collections::HashMap;

use speedy2d::{
    color::Color,
    image::{ImageDataType, ImageFileFormat, ImageHandle},
    shape::Rectangle,
};

use crate::{utility::animation::{Animation, AnimationSelectError}, world::space::GamePos};

use super::Entity;

pub struct Player<'a> {
    pos: GamePos,
    anim: Animation<'a>,
    screen_size: (f32, f32),
}

impl<'a> Entity for Player<'a> {
    fn draw(&mut self, graphics: &mut speedy2d::Graphics2D) {
        self.anim.draw(
            graphics,
            Rectangle::from_tuples(
                self.pos.into(),
                (
                    self.pos.x + self.screen_size.0,
                    self.pos.y + self.screen_size.1,
                ),
            ),
            Color::WHITE,
        );
    }
    fn moove(&mut self, change_pos: (f32, f32)) {
        self.pos = (self.pos.x + change_pos.0, self.pos.y + change_pos.1).into();
    }
    fn set_anim(&mut self, name: &str) -> Result<(), AnimationSelectError> {
        self.anim.select(name)
    }
    fn remove_anim(&mut self) {
        self.anim.deselect();
    }
}

impl<'a> Player<'a> {
    pub fn new(src: ImageHandle) -> Player<'a> {
        let mut frames: HashMap<&'a str, (bool, Vec<(u16, u16)>)> = HashMap::new();

        frames.insert("left", (true, vec![(0, 0), (1, 0)]));
        frames.insert("right", (true, vec![(0, 1), (1, 1)]));

        let anim = Animation::new(src, (5, 5), frames, (2, 0), 500);
        Player {
            pos: (300.0, 300.0).into(),
            anim,
            screen_size: (80.0, 80.0),
        }
    }
}
