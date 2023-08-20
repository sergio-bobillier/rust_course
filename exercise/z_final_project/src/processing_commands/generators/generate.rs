use image::DynamicImage;
use image::Rgb;
use rand::Rng;

use crate::processing_commands::ProcessingCommand;

pub struct Generate {
    width: u32,
    height: u32,
    color: [u8; 3],
    noise: bool
}

impl Generate {
    pub fn new(width: u32, height: u32, color: [u8; 3], noise: bool) -> Self {
        Self { height, width, color, noise }
    }
}

impl ProcessingCommand for Generate {
    fn name(&self) -> String { "Generate".to_string() }

    fn run(&self, _image: DynamicImage) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let buffer = image.as_mut_rgb8().unwrap();

        let (mut lbound, mut ubound): (i16, i16) = (0, 0);
        let mut rng = rand::thread_rng();
        let mut pixel_color = self.color.clone();

        if self.noise {
            let max = *pixel_color.iter().max().unwrap();
            let min = *pixel_color.iter().min().unwrap();
            ubound = 255 - (max as i16);
            lbound = 0 - (min as i16);
        }

        for x in 0..self.width {
            for y in 0..self.height {
                if self.noise {
                    pixel_color = self.color.clone();
                    let delta = rng.gen_range(lbound..=ubound);

                    for i in 0..3 {
                        let component = pixel_color[i] as i16;
                        let new_component = component + delta;
                        pixel_color[i] = (new_component) as u8;
                    }
                }

                buffer.put_pixel(x, y, Rgb(pixel_color));
            }
        }

        image
    }

    fn description(&self) -> String {
        let noisy = if self.noise { "noisy " } else { "" };

        format!(
            "Generated a {}{} by {} pixels image with color {:?}...",
            noisy, self.width, self.height, self.color
        )
    }
}