use core::panic;
use std::{
    collections::{HashMap, HashSet},
    rc::Weak,
    sync::atomic::Ordering,
    time::Instant,
};

use bitflags::bitflags;
use speedy2d::{
    color::Color,
    image::{ImageDataType, ImageFileFormat, ImageSmoothingMode},
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{
    entity::{player::Player, Entity},
    utility::animation::AnimationSelectError,
};

use super::{title::TitleScreen, RedirectHandler, Screen, RESOLUTION};

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

pub struct GameScreen<'a> {
    new_screen: Option<Box<dyn Screen>>,
    entities: HashMap<&'a str, Box<dyn Entity>>,
    current_input: Input,
}

impl<'a> WindowHandler<String> for GameScreen<'a> {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        if self.entities.get("player").is_none() {
            self.entities.insert(
                "player",
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
        }

        let current_input = self.current_input;

        let player = self.entities.get_mut("player").unwrap();

        if current_input.is_empty() {
            player.remove_anim();
        } else {
            player.moove(
                if check_input(current_input, Input::LEFT) { (-10.0, 0.0)}
                else if check_input(current_input, Input::RIGHT) { (10.0, 0.0) }
                else if check_input(current_input, Input::DOWN) { (0.0, 10.0) }
                else if check_input(current_input, Input::UP) { (0.0, -10.0) }
                else { (0.0, 0.0) }
            );
            if let Err(AnimationSelectError::NotFound) =
                player.set_anim(if check_input(current_input, Input::ATTACK) {
                    "attack"
                } else {
                    "move"
                })
            {
                panic!("No animation found");
            }
        }

        graphics.clear_screen(Color::CYAN);

        for (_, entity) in self.entities.iter_mut() {
            entity.draw(graphics);
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
            let player = self.entities.get_mut("player");
            match virtual_key_code {
                VirtualKeyCode::Escape => {
                    self.new_screen = Some(Box::new(TitleScreen::new()));
                }
                _ => {
                    self.current_input |= match virtual_key_code {
                        VirtualKeyCode::Up => Input::UP,
                        VirtualKeyCode::Down => Input::DOWN,
                        VirtualKeyCode::Right => Input::RIGHT,
                        VirtualKeyCode::Left => Input::LEFT,
                        VirtualKeyCode::Space => Input::ATTACK,
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
            let player = self.entities.get_mut("player");
            match virtual_key_code {
                VirtualKeyCode::Right => {
                    self.current_input &= !Input::RIGHT;
                    if let Some(player) = player {
                        player.remove_anim();
                    }
                }
                VirtualKeyCode::Left => {
                    self.current_input &= !Input::LEFT;
                    if let Some(player) = player {
                        player.remove_anim();
                    }
                }
                VirtualKeyCode::Up => {
                    self.current_input &= !Input::UP;
                }
                VirtualKeyCode::Down => {
                    self.current_input &= !Input::DOWN;
                }
                VirtualKeyCode::Space => {
                    self.current_input &= !Input::ATTACK;
                    if let Some(player) = player {
                        player.remove_anim();
                    }
                }
                _ => (),
            }
        }
    }
}

impl<'a> Screen for GameScreen<'a> {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
}

impl<'a> GameScreen<'a> {
    pub fn new() -> GameScreen<'a> {
        GameScreen {
            new_screen: None,
            entities: HashMap::new(),
            current_input: Input { bits: 0 },
        }
    }
}

fn check_input(flag: Input, comp_flag: Input) -> bool {
    flag & comp_flag == comp_flag
}
