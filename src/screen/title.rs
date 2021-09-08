use std::{
    collections::{HashMap, HashSet},
    sync::atomic::Ordering,
};

use speedy2d::{Graphics2D, color::Color, font::{Font, TextAlignment, TextLayout, TextOptions}, shape::Rectangle, window::{MouseButton, UserEventSender, VirtualKeyCode, WindowHandler, WindowHelper}};

use crate::{
    screen::RESOLUTION,
    ui::{button::Button, rect::rect_from_size},
};

use super::{RedirectHandler, Screen, game::GameScreen};

pub struct TitleScreen<'a> {
    new_screen: Option<Box<dyn Screen>>,
    mouse_up: bool,
    buttons: HashMap<&'a str, Button<'a>>,
    mouse_pos: (f32, f32),
    user_event_sender: Option<UserEventSender<String>>,
}

impl<'a> WindowHandler<String> for TitleScreen<'a> {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        if self.user_event_sender.is_none() {
            self.user_event_sender = Some(helper.create_user_event_sender());
        }

        graphics.clear_screen(Color::BLUE);

        self.buttons.get("start").unwrap().draw(graphics);

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
                    self.new_screen = Some(Box::new(GameScreen::new()));
                }
                _ => (),
            }
        }
    }
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<String>, position: speedy2d::dimen::Vector2<f32>) {
        self.mouse_pos = (position.x, position.y);
    }
    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        button: speedy2d::window::MouseButton,
    ) {
        self.mouse_up = true;
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<String>, button: MouseButton) {
        if self.mouse_up {
            if let MouseButton::Left = button {
                for (_, button) in self.buttons.iter() {
                    if button.in_bounds(self.mouse_pos) {
                        button.click(&self.user_event_sender.as_ref().unwrap());
                    }
                }
            }
        }
        self.mouse_up = false;
    }
    fn on_resize(
        &mut self,
        helper: &mut WindowHelper<String>,
        size_pixels: speedy2d::dimen::Vector2<u32>,
    ) {
        let start_button = self.buttons.get_mut("start").unwrap();
        RESOLUTION.0.store(size_pixels.x, Ordering::Relaxed);
        RESOLUTION.1.store(size_pixels.y, Ordering::Relaxed);
        start_button
            .set_bounds(rect_from_size(
                start_button.width(),
                start_button.height(),
                (
                    RESOLUTION.0.load(Ordering::Relaxed) / 2,
                    RESOLUTION.1.load(Ordering::Relaxed) / 2,
                ),
            ));
    }
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<String>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
    }
    fn on_user_event(&mut self, helper: &mut WindowHelper<String>, user_event: String) {
        match &user_event[..] {
            "start" => {
                self.new_screen = Some(Box::new(GameScreen::new()));
            }
            _ => (),
        }
    }
}

impl<'a> Screen for TitleScreen<'a> {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
}

impl<'a> TitleScreen<'a> {
    pub fn new() -> TitleScreen<'a> {
        let font = Font::new(include_bytes!("../../assets/font/Cabal-w5j3.ttf")).unwrap();

        let mut buttons = HashMap::new();

        let res = (RESOLUTION.0.load(Ordering::Relaxed), RESOLUTION.1.load(Ordering::Relaxed));

        buttons.insert(
            "start",
            Button::new(
                "start",
                64.0,
                Box::new(|s: &UserEventSender<String>| {
                    s.send_event(String::from("start")).unwrap();
                }),
                180,
                60,
                (
                    res.0 / 2,
                    res.1 / 2,
                ),
                Color::WHITE,
                Color::BLACK,
                font,
            ),
        );

        TitleScreen {
            new_screen: None,
            mouse_up: true,
            buttons,
            mouse_pos: (0.0, 0.0),
            user_event_sender: None,
        }
    }
}
