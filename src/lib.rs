use speedy2d::Window;

mod entity;
mod screen;
mod world;
mod utility;

pub fn run() {
    let window = Window::new_centered("Goblins", (800, 600)).unwrap();

    window.run_loop(screen::RedirectHandler::new(Box::new(
        screen::title::TitleScreen::new(),
    )));
}
