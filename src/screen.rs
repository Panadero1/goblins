use std::{rc::Weak, sync::atomic::AtomicU32};

use speedy2d::{
    dimen::Vector2,
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

pub mod game;
pub mod title;

pub static RESOLUTION: (AtomicU32, AtomicU32) = (AtomicU32::new(640), AtomicU32::new(480));

pub trait Screen: WindowHandler<String> {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>>;
}

pub struct RedirectHandler {
    my_handler: Box<dyn Screen>,
}
impl WindowHandler<String> for RedirectHandler {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<String>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
        self.my_handler.on_start(helper, info);
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<String>, user_event: String) {
        self.my_handler.on_user_event(helper, user_event);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<String>, size_pixels: Vector2<u32>) {
        self.my_handler.on_resize(helper, size_pixels);
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<String>, scale_factor: f64) {
        self.my_handler
            .on_scale_factor_changed(helper, scale_factor);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        if let Some(new_screen) = self.my_handler.change_screen() {
            self.my_handler = new_screen;
        }
        self.my_handler.on_draw(helper, graphics);

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<String>, position: Vector2<f32>) {
        self.my_handler.on_mouse_move(helper, position);
    }

    fn on_mouse_button_down(
        &mut self,
        helper: &mut WindowHelper<String>,
        button: speedy2d::window::MouseButton,
    ) {
        self.my_handler.on_mouse_button_down(helper, button);
    }

    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        button: speedy2d::window::MouseButton,
    ) {
        self.my_handler.on_mouse_button_up(helper, button);
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<String>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        self.my_handler
            .on_key_down(helper, virtual_key_code, scancode);
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        self.my_handler
            .on_key_up(helper, virtual_key_code, scancode);
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<String>, unicode_codepoint: char) {
        self.my_handler.on_keyboard_char(helper, unicode_codepoint);
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<String>,
        state: speedy2d::window::ModifiersState,
    ) {
        self.my_handler.on_keyboard_modifiers_changed(helper, state);
    }
}

impl RedirectHandler {
    pub fn new(my_handler: Box<dyn Screen>) -> RedirectHandler {
        RedirectHandler { my_handler }
    }
}
