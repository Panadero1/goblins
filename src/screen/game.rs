use std::{collections::{HashMap, HashSet}, rc::Weak, sync::atomic::Ordering, time::Instant};

use speedy2d::{
    color::Color,
    image::{ImageDataType, ImageFileFormat, ImageSmoothingMode},
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::entity::{player::Player, Entity};

use super::{title::TitleScreen, RedirectHandler, Screen, RESOLUTION};

pub struct GameScreen<'a> {
    new_screen: Option<Box<dyn Screen>>,
    entities: HashMap<&'a str, Box<dyn Entity>>,
    can_anim: [bool; 4], // move, fight, etc, etc
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
                    }
                }
                VirtualKeyCode::Down => {
                    if let Some(player) = player {
                        player.moove((0.0, 10.0));
                    }
                }
                VirtualKeyCode::Right => {
                    if let Some(player) = player {
                        player.moove((10.0, 0.0));
                        if self.can_anim[0] {
                            player.set_anim("move").unwrap();
                            self.can_anim = [false, true, true, true];
                        }
                    }
                }
                VirtualKeyCode::Left => {
                    if let Some(player) = player {
                        player.moove((-10.0, 0.0));
                        if self.can_anim[0] {
                            player.set_anim("move").unwrap();
                            self.can_anim = [false, true, true, true];
                        }
                    }
                },
                VirtualKeyCode::Space => {
                    if let Some(player) = player {
                        if self.can_anim[1] {
                            player.set_anim("attack").unwrap();
                            self.can_anim = [true, false, false, false];
                        }
                    }
                },
                _ => (),
            }
        }
    }
    fn on_key_up(&mut self, helper: &mut WindowHelper<String>, virtual_key_code: Option<VirtualKeyCode>, scancode: speedy2d::window::KeyScancode) {
        
        if let Some(virtual_key_code) = virtual_key_code {
            let player = self.entities.get_mut("player");
            match virtual_key_code {
                VirtualKeyCode::Right => {
                    if let Some(player) = player {
                        player.remove_anim();
                        self.can_anim = [true, true, true, true];
                    }
                },
                VirtualKeyCode::Left => {
                    if let Some(player) = player {
                        player.remove_anim();
                        self.can_anim = [true, true, true, true];
                    }
                },
                VirtualKeyCode::Space => {
                    if let Some(player) = player {
                        player.remove_anim();
                        self.can_anim = [true, true, true, true];
                    }
                },
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
            can_anim: [true; 4],
        }
    }
}
