use image::DynamicImage;

use crate::processing_commands::ProcessingCommand;

const DEFAULT_WIDTH:u32 = 800;
const DEFAULT_HEIGHT:u32 = 800;

pub struct Fractal {
    width: u32,
    height: u32
}

impl Fractal {
    pub fn new() -> Self {
        Self { width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT }
    }
}

impl ProcessingCommand for Fractal {
    fn name(&self) -> String { "Fractal".to_string() }

    fn run(&self, _imge: DynamicImage) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let buffer = image.as_mut_rgb8().unwrap();

        let scale_x = 3.0 / self.width as f32;
        let scale_y = 3.0 / self.height as f32;

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            // Use red and blue to be a pretty gradient background
            let red = (0.3 * x as f32) as u8;
            let blue = (0.3 * y as f32) as u8;

            // Use green as the fractal foreground (here is the fractal math part)
            let cx = y as f32 * scale_x - 1.5;
            let cy = x as f32 * scale_y - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut green = 0;
            while green < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                green += 1;
            }

            // Actually set the pixel. red, green, and blue are u8 values!
            *pixel = image::Rgb([red, green, blue]);
        }

        image
    }

    fn description(&self) -> String {
        format!("Generated {}x{} fractal", self.width, self.height)
    }
}