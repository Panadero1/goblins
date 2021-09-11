use std::{
    collections::{HashMap, HashSet},
    rc::Weak,
    sync::atomic::Ordering,
};

use speedy2d::{
    color::Color,
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::entity::{player::Player, Entity};

use super::{title::TitleScreen, RedirectHandler, Screen, RESOLUTION};

pub struct GameScreen<'a> {
    new_screen: Option<Box<dyn Screen>>,
    entities: HashMap<&'a str, Box<dyn Entity>>,
}

impl<'a> WindowHandler<String> for GameScreen<'a> {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::CYAN);

        for (_, entity) in self.entities.iter() {
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
            match virtual_key_code {
                VirtualKeyCode::Space => {
                    self.new_screen = Some(Box::new(TitleScreen::new()));
                }
                VirtualKeyCode::Up => {
                    self.entities.get_mut("player").unwrap().moove((0.0, -10.0));
                }
                VirtualKeyCode::Down => {
                    self.entities.get_mut("player").unwrap().moove((0.0, 10.0));
                }
                VirtualKeyCode::Right => {
                    self.entities.get_mut("player").unwrap().moove((10.0, 0.0));
                }
                VirtualKeyCode::Left => {
                    self.entities.get_mut("player").unwrap().moove((-10.0, 0.0));
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
        let mut entities: HashMap<&'a str, Box<dyn Entity>> = HashMap::new();

        entities.insert(
            "player",
            Box::new(Player::new())
        );

        GameScreen {
            new_screen: None,
            entities,
        }
    }
}
