use std::sync::atomic::Ordering;

use screen::RESOLUTION;
use speedy2d::{Window, dimen::Vector2, window::{WindowCreationOptions, WindowPosition, WindowSize}};

mod entity;
mod screen;
mod ui;
mod world;

pub fn run() {
    let window: Window<String> = Window::new_with_user_events(
        "Goblins",
        WindowCreationOptions::new_windowed(
            WindowSize::PhysicalPixels(Vector2::new(
                RESOLUTION.0.load(Ordering::Relaxed),
                RESOLUTION.1.load(Ordering::Relaxed),
            )),
            Some(WindowPosition::Center),
        ),
    )
    .unwrap();

    window.run_loop(screen::RedirectHandler::new(Box::new(
        screen::title::TitleScreen::new(),
    )));
}
