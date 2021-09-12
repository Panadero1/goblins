use std::{collections::{HashMap, HashSet}, rc::Weak, sync::atomic::Ordering, time::Instant};

use bitflags::bitflags;
use speedy2d::{
    color::Color,
    image::{ImageDataType, ImageFileFormat, ImageSmoothingMode},
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{entity::{player::Player, Entity}, utility::animation::AnimationSelectError};

use super::{title::TitleScreen, RedirectHandler, Screen, RESOLUTION};

bitflags! {
    struct Input: u8 {
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
                VirtualKeyCode::Up => {
                    if let Some(player) = player {
                        player.moove((0.0, -10.0));
                        if self.current_input & Input::UP != Input::UP {
                            // if let Err(AnimationSelectError::NotFound) = player.set_anim("up") {
                            //     panic!("No animation found");
                            // }
                        }
                    }
                    self.current_input |= Input::UP;
                }
                VirtualKeyCode::Down => {
                    if let Some(player) = player {
                        player.moove((0.0, 10.0));
                        if self.current_input & Input::DOWN != Input::DOWN {
                            // if let Err(AnimationSelectError::NotFound) = player.set_anim("down") {
                            //     panic!("No animation found");
                            // }
                        }
                    }
                    self.current_input |= Input::DOWN;
                }
                VirtualKeyCode::Right => {
                    if let Some(player) = player {
                        player.moove((10.0, 0.0));
                        if self.current_input & Input::RIGHT != Input::RIGHT {
                            if let Err(AnimationSelectError::NotFound) = player.set_anim("right") {
                                panic!("No animation found");
                            }
                        }
                    }
                    self.current_input |= Input::RIGHT;
                }
                VirtualKeyCode::Left => {
                    if let Some(player) = player {
                        player.moove((-10.0, 0.0));
                        if self.current_input & Input::LEFT != Input::LEFT {
                            if let Err(AnimationSelectError::NotFound) = player.set_anim("left") {
                                panic!("No animation found");
                            }
                        }
                    }
                    self.current_input |= Input::LEFT;
                }
                _ => (),
            }
        }
    }
    fn on_key_up(&mut self, helper: &mut WindowHelper<String>, virtual_key_code: Option<VirtualKeyCode>, scancode: speedy2d::window::KeyScancode) {
        
        if let Some(virtual_key_code) = virtual_key_code {
            let player = self.entities.get_mut("player");
            match virtual_key_code {
                VirtualKeyCode::Right => {
                    self.current_input &= !Input::RIGHT;
                    if let Some(player) = player {
                        player.remove_anim();
                    }
                },
                VirtualKeyCode::Left => {
                    self.current_input &= !Input::LEFT;
                    if let Some(player) = player {
                        player.remove_anim();
                    }
                },
                VirtualKeyCode::Up => {
                    self.current_input &= !Input::UP;
                }
                VirtualKeyCode::Down => {
                    self.current_input &= !Input::DOWN;
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
