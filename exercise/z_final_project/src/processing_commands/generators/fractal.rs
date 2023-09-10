use image::DynamicImage;

use crate::processing_commands::ProcessingCommand;
use crate::size::Size;
use crate::overlay::Overlay;

pub struct Fractal {
    size: Size,
    overlay: Option<Overlay>
}

impl Fractal {
    pub fn new(size: Size, overlay: Option<Overlay>) -> Self {
        Self { size, overlay }
    }
}

impl ProcessingCommand for Fractal {
    fn name(&self) -> String { "Fractal".to_string() }

    fn pre_process(&mut self, image: &DynamicImage) {
        self.size.resolve(image);
    }

    fn run(&self, original: DynamicImage) -> DynamicImage {
        let width = self.size.width();
        let height = self.size.height();
        let mut image = DynamicImage::new_rgb8(width, height);
        let buffer = image.as_mut_rgb8().unwrap();

        let scale_x = 3.0 / width as f32;
        let scale_y = 3.0 / height as f32;

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

        match &self.overlay {
            Some(overlay) => { overlay.run(original, image) },
            None => image
        }
    }

    fn description(&self) -> String {
        format!("Generated {} fractal", self.size)
    }
}