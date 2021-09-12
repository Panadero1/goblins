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
    direction_left: bool,
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
        self.direction_left = change_pos.0 < 0.0;
        self.pos = (self.pos.0 + change_pos.0, self.pos.1 + change_pos.1);
    }
    fn set_anim(&mut self, name: &str) -> Result<(), ()> {
        let name = match name {
            "move" => if self.direction_left {"move left"} else {"move right"},
            "attack" => if self.direction_left {"attack left"} else {"attack right"},
            _ => return Err(())
        };
        self.anim.select(name)
    }
    fn remove_anim(&mut self) {
        self.anim.deselect();
    }
}

impl<'a> Player<'a> {
    pub fn new(src: ImageHandle) -> Player<'a> {
        let mut frames: HashMap<&'a str, (bool, Vec<(u16, u16)>)> = HashMap::new();

        frames.insert("attack right", (true, vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]));
        frames.insert("attack left", (true, vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)]));
        frames.insert("move left", (true, vec![(3, 0), (4, 0)]));
        frames.insert("move right", (true, vec![(1, 0), (2, 0)]));

        let anim = Animation::new(src, (8, 10), frames, (0, 0), 100);
        Player {
            pos: (300.0, 300.0).into(),
            anim,
            screen_size: (80.0, 80.0),
            direction_left: true,
        }
    }
}
