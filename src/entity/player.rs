use speedy2d::image::{ImageDataType, ImageFileFormat, ImageHandle};

use super::Entity;

pub struct Player {
    pos: (f32, f32),
}

impl Entity for Player {
    fn draw(&self, graphics: &mut speedy2d::Graphics2D) {
        let image = graphics.create_image_from_file_path(
            Some(ImageFileFormat::PNG),
            speedy2d::image::ImageSmoothingMode::Linear,
            ".\\assets\\img\\hero.png",
        ).expect("img path is busted");
        graphics.draw_image(self.pos, &image);
    }
    fn moove(&mut self, change_pos: (f32, f32)) {
        self.pos = (self.pos.0 + change_pos.0, self.pos.1 + change_pos.1)
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: (300.0, 300.0),
        }
    }
}
