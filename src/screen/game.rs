use std::{rc::Weak, sync::atomic::Ordering};

use speedy2d::{Graphics2D, color::Color, window::{VirtualKeyCode, WindowHandler, WindowHelper}};

use super::{RESOLUTION, RedirectHandler, Screen, title::TitleScreen};

pub struct GameScreen {
    new_screen: Option<Box<dyn Screen>>,
    mouse_pos: (f32, f32)
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::RED);

        helper.request_redraw();
    }
    fn on_key_down(&mut self, helper: &mut WindowHelper<String>, virtual_key_code: Option<speedy2d::window::VirtualKeyCode>, scancode: speedy2d::window::KeyScancode) {
        if let Some(virtual_key_code) = virtual_key_code {
            
            match virtual_key_code {
                VirtualKeyCode::Space => {
                    self.new_screen = Some(Box::new(TitleScreen::new()));
                }
                _ => (),
            }
        }
    }
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<String>, position: speedy2d::dimen::Vector2<f32>) {
        self.mouse_pos = (position.x, position.y);
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
        GameScreen {
            new_screen: None,
            mouse_pos: (0.0, 0.0)
        }
    }
}