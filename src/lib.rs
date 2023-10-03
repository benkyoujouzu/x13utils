use rusttype::{Font, Scale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use std::path::Path;

pub struct NumberImage {
    font: Font<'static>,
    pub image: RgbaImage
}

impl NumberImage {
    pub fn new() -> NumberImage {
        let font = Vec::from(include_bytes!("C:\\Windows\\Fonts\\Arial.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();
        NumberImage { font, image: RgbaImage::new(64, 64) }
    }

    pub fn save(&self) {
        let path = Path::new("./result.png");
        self.image.save(path).unwrap();
    }

    pub fn update(&mut self, num: f32) {
        let num_string;
        if num >= 9.95 || num <= -9.95 {
            num_string = format!("{}", num.round() as i32);
        } else {
            num_string = format!("{:.1}", num);
        }
        let icon_width = 64.0;
        let height = 40.0;
        let scale = Scale {
            x: height,
            y: height
        };
        let (width, _) = text_size(scale, &self.font, &num_string);
        let x = ((icon_width - width as f32) / 2.0) as i32;
        let y = ((icon_width - height) / 2.0) as i32;
        let color = Rgba([255u8, 255u8, 255u8, 255u8]);
        self.image.fill(0);
        draw_text_mut(&mut self.image, color, x, y, scale, &self.font, &num_string);

    }

}
