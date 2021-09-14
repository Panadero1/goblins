use core::panic;
use std::{
    collections::{HashMap, HashSet},
    rc::Weak,
    sync::atomic::Ordering,
    time::Instant,
};

use bitflags::bitflags;
use rand::Rng;
use speedy2d::{
    color::Color,
    image::{ImageDataType, ImageFileFormat, ImageSmoothingMode},
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{entity::{self, Entity, player::Player, tile::Tile}, utility::{animation::AnimationSelectError, serial_namer::SerialNamer}};

use super::{RESOLUTION, RedirectHandler, Screen, camera::Camera, get_resolution, title::TitleScreen};

const SPEED: f32 = 0.15;

const JUMP: f32 = 5.0;

pub const GRAVITY: f32 = 0.2;

pub const DRAG: f32 = 0.1;

bitflags! {
    struct Input: u8 {
        const NONE   = 0b00000000;
        const LEFT   = 0b00000001;
        const RIGHT  = 0b00000010;
        const UP     = 0b00000100;
        const DOWN   = 0b00001000;
        const ATTACK = 0b00010000;
    }
}

pub struct GameScreen {
    new_screen: Option<Box<dyn Screen>>,
    background: Option<HashMap<String, Box<dyn Entity>>>,
    entities: Option<HashMap<String, Box<dyn Entity>>>,
    current_input: Input,
    camera: Camera,
    namer: SerialNamer,
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::CYAN);
        if self.entities.is_none() {
            self.init_sprites(graphics);
        }

        if let Some(entities) = &mut self.entities {
            if let Some(background) = &mut self.background {
                let current_input = self.current_input;

                let player = entities.get_mut("player").unwrap();
    
                if current_input.is_empty() {
                    player.remove_anim();
                } else {
                    let mut mvmt = if check_input(current_input, Input::LEFT) {
                        (-SPEED, 0.0)
                    } else if check_input(current_input, Input::RIGHT) {
                        (SPEED, 0.0)
                    } else if check_input(current_input, Input::DOWN) {
                        (0.0, JUMP)
                    } else {
                        (0.0, 0.0)
                    };

                    if check_input(current_input, Input::UP) && player.get_pos().y == 0.0 {
                        mvmt.1 = -JUMP;
                    }

                    player.accelerate(mvmt.into());
                    if let Err(AnimationSelectError::NotFound) =
                        player.intercept_anim(if check_input(current_input, Input::ATTACK) {
                            "attack"
                        } else {
                            "move"
                        })
                    {
                        panic!("No animation found");
                    }
                }

                // This leads to the camera always being *slightly* behind the player (especially if player is moving fast)
                self.camera.pos = (player.get_pos().x, 0.0).into();
    
                for (_, background_object) in background.iter_mut() {
                    background_object.draw(graphics, &self.camera);
                }
    
                for (_, entity) in entities.iter_mut() {
                    entity.draw(graphics, &self.camera);
                }
            }
        }
        helper.request_redraw();
    }
    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<String>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(virtual_key_code) = virtual_key_code {
            match virtual_key_code {
                VirtualKeyCode::Escape => {
                    self.new_screen = Some(Box::new(TitleScreen::new()));
                }
                _ => {
                    self.current_input |= match virtual_key_code {
                        VirtualKeyCode::Left => Input::LEFT,
                        VirtualKeyCode::Up => Input::UP,
                        VirtualKeyCode::Down => Input::DOWN,
                        VirtualKeyCode::Right => Input::RIGHT,
                        VirtualKeyCode::X => Input::ATTACK,
                        _ => Input::NONE,
                    }
                }
            }
        }
    }
    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(virtual_key_code) = virtual_key_code {
            self.current_input &= !match virtual_key_code {
                VirtualKeyCode::Right => Input::RIGHT,
                VirtualKeyCode::Left => Input::LEFT,
                VirtualKeyCode::Up => Input::UP,
                VirtualKeyCode::Down => Input::DOWN,
                VirtualKeyCode::X => Input::ATTACK,
                _ => Input::NONE,
            }
        }
    }
    fn on_resize(
        &mut self,
        helper: &mut WindowHelper<String>,
        size_pixels: speedy2d::dimen::Vector2<u32>,
    ) {
        self.camera.width = size_pixels.x as f32 / 10.0;
        self.camera.height = size_pixels.y as f32 / 10.0;
    }
}

impl Screen for GameScreen {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
}

impl GameScreen {
    pub fn new() -> GameScreen {
        let res = get_resolution();
        GameScreen {
            new_screen: None,
            entities: None,
            background: None,
            current_input: Input { bits: 0 },
            camera: Camera::new((0.0, 0.0).into(), res.0 as f32 / 10.0, res.1 as f32 / 10.0),
            namer: SerialNamer::new(),
        }
    }
    fn init_sprites(&mut self, graphics: &mut Graphics2D) {
        let mut entities: HashMap<String, Box<dyn Entity>> = HashMap::new();
        let mut background: HashMap<String, Box<dyn Entity>> = HashMap::new();

        entities.insert(
            "player".to_string(),
            Box::new(Player::new(
                graphics
                    .create_image_from_file_path(
                        Some(ImageFileFormat::PNG),
                        ImageSmoothingMode::NearestNeighbor,
                        ".\\assets\\img\\knight.png",
                    )
                    .unwrap(),
            )),
        );

        let mut r = rand::thread_rng();

        for i in -10..20 {
            let display = (r.gen_range(0..4), r.gen_range(0..4));

            background.insert(
                self.namer.gen_name(),
                Box::new(Tile::new(
                    graphics
                        .create_image_from_file_path(
                            Some(ImageFileFormat::PNG),
                            ImageSmoothingMode::NearestNeighbor,
                            ".\\assets\\img\\tiles.png",
                        )
                        .unwrap(),
                    display,
                    ((i as f32) * 5.0, 10.0),
                )),
            );
        }

        self.entities = Some(entities);
        self.background = Some(background);
    }
}

fn check_input(flag: Input, comp_flag: Input) -> bool {
    flag & comp_flag == comp_flag
}
