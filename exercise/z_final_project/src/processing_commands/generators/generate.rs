use image::DynamicImage;
use image::Rgb;
use rand::Rng;

use crate::processing_commands::ProcessingCommand;
use crate::size::Size;

pub struct Generate {
    size: Size,
    color: [u8; 3],
    noise: bool
}

impl Generate {
    pub fn new(size: Size, color: [u8; 3], noise: bool) -> Self {
        Self { size, color, noise }
    }
}

impl ProcessingCommand for Generate {
    fn name(&self) -> String { "Generate".to_string() }

    fn pre_process(&mut self, image: &DynamicImage) {
        self.size.resolve(image);
    }

    fn run(&self, _image: DynamicImage) -> DynamicImage {
        let width = self.size.width();
        let height = self.size.height();

        let mut image = DynamicImage::new_rgb8(width, height);
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

        for x in 0..width {
            for y in 0..height {
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
            "Generated a {}{} pixels image with color {:?}...",
            noisy, self.size, self.color
        )
    }
}