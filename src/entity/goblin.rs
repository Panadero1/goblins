use std::{cmp::Ordering, collections::HashMap};

use speedy2d::{
    color::Color,
    image::{ImageDataType, ImageFileFormat, ImageHandle, ImageSmoothingMode},
    shape::Rectangle,
    Graphics2D,
};

use crate::{
    screen::{
        camera::Camera,
        game::{self, DRAG},
    },
    utility::animation::{Animation, AnimationSelectError},
    world::space::GamePos,
};

use super::Entity;

// Consts

const SPEED: f32 = 0.1;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

pub struct Goblin {
    pub pos: GamePos,
    anim: Animation,
    game_size: (f32, f32),
    direction: Direction,
    pub velocity: GamePos,
    pub attacking: bool,
}

impl Entity for Goblin {
    fn draw(&mut self, graphics: &mut speedy2d::Graphics2D, camera: &Camera) {
        self.update();
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
        self.velocity = (change_pos.0, change_pos.1).into();
    }
    fn set_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        self.anim.select(&self.map_animation(anim_name)[..])
    }
    fn intercept_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        // println!("{}", anim_name);
        self.anim.intercept(&self.map_animation(anim_name)[..])

    }
    fn remove_anim(&mut self) {
        self.anim.deselect();
    }
    fn accelerate(&mut self, vector: GamePos) {
        self.velocity += vector * SPEED;
    }
    fn get_pos(&self) -> GamePos {
        self.pos
    }
}

impl Goblin {
    pub fn new(graphics: &mut Graphics2D) -> Goblin {
        let src = graphics
            .create_image_from_file_path(
                Some(ImageFileFormat::PNG),
                ImageSmoothingMode::NearestNeighbor,
                ".\\assets\\img\\goblin.png",
            )
            .unwrap();

        let mut frames: HashMap<&'static str, (bool, Vec<(u16, u16)>)> = HashMap::with_capacity(8);

        frames.insert(
            "attack right",
            (true, vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]),
        );
        frames.insert(
            "attack left",
            (true, vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)]),
        );
        frames.insert("move right", (true, vec![(0, 0), (1, 0), (2, 0), (3, 0)]));
        frames.insert("move left", (true, vec![(0, 1), (1, 1), (2, 1), (3, 1)]));
        frames.insert("idle right", (true, vec![(0, 0)]));
        frames.insert("idle left", (true, vec![(0, 1)]));
        frames.insert("attack right", (false, vec![(0, 2), (1, 2), (2, 2), (3, 2)]));
        frames.insert("attack left", (false, vec![(0, 3), (1, 3), (2, 3), (3, 3)]));

        let anim = Animation::new(src, (10, 10), frames, (0, 0), 100);
        Goblin {
            pos: (0.0, 0.0).into(),
            anim,
            game_size: (10.0, 10.0),
            direction: Direction::Right,
            velocity: (0.0, 0.0).into(),
            attacking: false,
        }
    }
    fn update(&mut self) {
        self.velocity.y += game::GRAVITY;
        self.velocity *= 1.0 - DRAG;
        self.pos += self.velocity;
        if self.pos.y > 0.0 {
            self.pos.y = 0.0;
            self.velocity.y = 0.0;
        }

        self.direction = match self.velocity.x.partial_cmp(&0.0) {
            Some(Ordering::Equal) => self.direction,
            Some(Ordering::Greater) => Direction::Right,
            Some(Ordering::Less) => Direction::Left,
            None => self.direction,
        };
        if self.anim.frame_loop.is_none() {
            self.attacking = false;
        }
        if let Err(AnimationSelectError::NotFound) =
            self.set_anim(if self.attacking {
                "attack"
            } else if self.velocity.x.abs() < 0.01 {
                "idle"
            } else {
                "move"
            })
        {
            panic!("Animation not found..");
        }
    }
    fn map_animation(&self, anim_name: &str) -> String {
        format!("{} {}", anim_name, match self.direction {Direction::Left => "left", Direction::Right => "right"})
    }
}
