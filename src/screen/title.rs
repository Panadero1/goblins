use speedy2d::{Graphics2D, color::Color, font::{Font, TextAlignment, TextLayout, TextOptions}, shape::Rectangle, window::{MouseButton, VirtualKeyCode, WindowHandler, WindowHelper}};

use crate::utility::text;

use super::{game::GameScreen, Screen};

pub struct TitleScreen {
    new_screen: Option<Box<dyn Screen>>,
    mouse_up: bool,
    start_button: ((f32, f32), (f32, f32)),
    font: Font,
}

impl WindowHandler for TitleScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLUE);

        graphics.draw_rectangle(
            Rectangle::from_tuples(self.start_button.0, self.start_button.1),
            Color::WHITE,
        );

        let text = "Start";

        let width = self.start_button.1.0 - self.start_button.0.0;

        
        graphics.draw_text(
            self.start_button.0,
            Color::BLACK,
            &self
                .font
                .layout_text(text, 32.0, TextOptions::new().with_wrap_to_width(width, TextAlignment::Center)),
        );
        helper.request_redraw();
    }
    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(virtual_key_code) = virtual_key_code {
            match virtual_key_code {
                VirtualKeyCode::Space => {
                    self.new_screen = Some(Box::new(GameScreen::new()));
                },
                _ => (),
            }
        }
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        if self.mouse_up {
            if let MouseButton::Left = button {}
        }
        self.mouse_up = false;
    }
    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        self.mouse_up = true;
    }
    fn on_resize(
        &mut self,
        helper: &mut WindowHelper<()>,
        size_pixels: speedy2d::dimen::Vector2<u32>,
    ) {
        println!("{}", size_pixels.x);
        self.start_button = (
            (size_pixels.x as f32 * 0.3, size_pixels.y as f32 * 0.4),
            (size_pixels.x as f32 * 0.7, size_pixels.y as f32 * 0.7),
        );
    }
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
        helper.set_size_pixels((600, 600));
    }
}

impl Screen for TitleScreen {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
}

impl TitleScreen {
    pub fn new() -> TitleScreen {
        TitleScreen {
            new_screen: None,
            mouse_up: false,
            start_button: ((0.0, 0.0), (0.0, 0.0)),
            font: Font::new(include_bytes!("../../assets/font/Cabal-w5j3.ttf")).unwrap(),
        }
    }
}
