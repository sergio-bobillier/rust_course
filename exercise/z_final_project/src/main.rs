// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{arg, command, value_parser, ArgAction, Command};
use image::{DynamicImage, Rgb};
use rand::Rng;

struct Geometry{
    top: u32,
    left: u32,
    height: Option<u32>,
    width: Option<u32>
}

const VALID_ANGLES:[u16; 3] = [90, 180, 270];

fn valid_angle(value: &str) -> Result<u16, String> {
    let angle = value.parse::<u16>().map_err(|_| format!("--angle must be an integer, '{value}' given"))?;

    if VALID_ANGLES.contains(&angle) {
        Ok(angle)
    } else {
        Err(
            format!("Valid angles are {:?}", VALID_ANGLES)
        )
    }
}

fn parse_color(value: Option<&String>) -> [u8; 3] {
    match value {
        Some(color) => {
            if color == "random" {
                let mut rng = rand::thread_rng();

                [
                    rng.gen(),
                    rng.gen(),
                    rng.gen()
                ]
            }
            else {
                let parts: Vec<&str> = color.split(",").map(|string| { string.trim() }).collect();

                if parts.len() != 3 {
                    panic!("The given color doesn't have the format: r, g, b (all integers betweem 0 and 255)");
                }

                [
                    parts[0].parse::<u8>().unwrap(),
                    parts[1].parse::<u8>().unwrap(),
                    parts[2].parse::<u8>().unwrap()
                ]
            }
        }
        None => {
            [0, 0, 0]
        }
    }
}

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/

    // * Blur
    // * Brighten
    // * Crop
    // * Rotate
    // * Invert
    // * Grayscale
    // * Fractsl

    let matches = command!().subcommand_required(true)
        .subcommand(
            Command::new("blur")
                .about("Blurs the given image")
                .arg(arg!(<INFILE> "Input file name"))
                .arg(arg!(<OUTFILE> "Output file name"))
                .arg(
                    arg!(--amount <VALUE> "how much to blur the image")
                    .value_parser(value_parser!(f32)).default_value("2.0")
                )
        )
        .subcommand(
            Command::new("crop")
                .about("Crops the given image to the given geometry")
                .arg(
                    arg!(--left <VALUE> "x-coordinate of the top-left corner of the cropped area")
                    .value_parser(value_parser!(u32)).default_value("0")
                )
                .arg(
                    arg!(--top <VALUE> "y-coordinate of top-left corner of the cropped area")
                    .value_parser(value_parser!(u32)).default_value("0")
                )
                .arg(
                    arg!(--width <VALUE> "width of the cropped area")
                    .value_parser(value_parser!(u32)).required(false)
                )
                .arg(
                    arg!(--height <VALUE> "height of the cropped area")
                    .value_parser(value_parser!(u32)).required(false)
                )
                .arg(arg!(<INFILE> "Input file name"))
                .arg(arg!(<OUTFILE> "Output file name"))
        )
        .subcommand(
            Command::new("brighten")
                .about("Brightens or dims the given image")
                .arg(
                    arg!(--delta <VALUE> "how much to brighten (positive) or dim (negative) the image")
                    .value_parser(value_parser!(i32)).required(true)
                )
                .arg(arg!(<INFILE> "Input file name"))
                .arg(arg!(<OUTFILE> "Output file name"))
        )
        .subcommand(
            Command::new("rotate")
                .about("Rotates the given image clockwise by the given angle")
                .arg(
                    arg!(--angle <VALUE> "what angle to use to rotate the image, valid values are 90, 180 and 270")
                    .value_parser(valid_angle).required(true)
                )
                .arg(arg!(<INFILE> "Input file name"))
                .arg(arg!(<OUTFILE> "Output file name"))
        )
        .subcommand(
            Command::new("invert")
                .about("Inverts the color of the given image")
                .arg(arg!(<INFILE> "Input file name"))
                .arg(arg!(<OUTFILE> "Output file name"))
        )
        .subcommand(
            Command::new("grayscale")
                .about("Converts the given image to a grayscale image")
                .arg(arg!(<INFILE> "Input file name"))
                .arg(arg!(<OUTFILE> "Output file name"))
        )
        .subcommand(
            Command::new("generate")
                .about("Generates a black image with the specified dimensions")
                .arg(
                    arg!(--width <VALUE> "width of the generated image")
                    .value_parser(value_parser!(u32)).required(true)
                )
                .arg(
                    arg!(--height <VALUE> "height of the generated image")
                    .value_parser(value_parser!(u32)).required(true)
                )
                .arg(arg!(--color <VALUE> "RGB-color of the generated image. Format: r, g, b"))
                .arg(
                    arg!(--noise "If given the image will be noisy instead of a solid color")
                    .action(ArgAction::SetTrue)
                )
                .arg(arg!(<OUTFILE> "Output file name"))
        )
        .subcommand(
            Command::new("fractal")
                .about("Generates a fractal image")
                .arg(arg!(<OUTFILE> "Output file name"))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("blur") {
        let input_file = matches.get_one::<String>("INFILE").expect("required").clone();
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();
        let amount = *matches.get_one::<f32>("amount").expect("has default value");
        blur(input_file, output_file, amount);
    }

    if let Some(matches) = matches.subcommand_matches("crop") {
        let input_file = matches.get_one::<String>("INFILE").expect("required").clone();
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();

        let geometry = Geometry{
            top: *matches.get_one::<u32>("top").expect("has default value"),
            left: *matches.get_one::<u32>("left").expect("has default value"),
            width: matches.get_one::<u32>("width").copied(),
            height: matches.get_one::<u32>("height").copied()
        };

        crop(input_file, output_file, geometry);
    }

    if let Some(matches) = matches.subcommand_matches("brighten") {
        let input_file = matches.get_one::<String>("INFILE").expect("required").clone();
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();
        let delta = *matches.get_one::<i32>("delta").expect("required");

        brighten(input_file, output_file, delta);
    }

    if let Some(matches) = matches.subcommand_matches("rotate") {
        let input_file = matches.get_one::<String>("INFILE").expect("required").clone();
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();
        let angle = *matches.get_one::<u16>("angle").expect("required");

        rotate(input_file, output_file, angle);
    }

    if let Some(matches) = matches.subcommand_matches("invert") {
        let input_file = matches.get_one::<String>("INFILE").expect("required").clone();
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();

        invert(input_file, output_file);
    }

    if let Some(matches) = matches.subcommand_matches("grayscale") {
        let input_file = matches.get_one::<String>("INFILE").expect("required").clone();
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();

        grayscale(input_file, output_file);
    }

    if let Some(matches) = matches.subcommand_matches("generate") {
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();
        let width = *matches.get_one::<u32>("width").expect("has default value");
        let height = *matches.get_one::<u32>("height").expect("has default value");
        let color = parse_color(matches.get_one::<String>("color"));
        let noise: bool = matches.get_flag("noise");

        generate(output_file, width, height, color, noise);
    }

    if let Some(matches) = matches.subcommand_matches("fractal") {
        let output_file = matches.get_one::<String>("OUTFILE").expect("required").clone();

        fractal(output_file);
    }
}

fn open_image(path: String) -> DynamicImage {
   let message = format!("Failed to open: {}", path);
    image::open(&path).expect(&message)
}

fn save_image(image: DynamicImage, path: String) {
    let message = format!("Failed to save to: {}", path);
    image.save(path).expect(&message);
}

fn blur(infile: String, outfile: String, amount: f32) {
    // Here's how you open an existing image file
    let img = open_image(infile);
    let img2 = img.blur(amount);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, delta: i32) {
    println!("Brightening {} by {}... Saving result to: {}", infile, delta, outfile);
    let original = open_image(infile);
    let changed = original.brighten(delta);
    save_image(changed, outfile);
}

fn crop(infile: String, outfile: String, geometry: Geometry) {
    let original = image::open(&infile).expect("Failed to open INFILE");

    let width = match(geometry.width) {
        Some(width) => {
            width
        }
        None => {
            original.width()
        }
    };

    let height = match(geometry.height) {
        Some(height) => {
            height
        }
        None => {
            original.height()
        }
    };

    println!("Cropping {} from ({}, {}) to ({}, {})...", infile, geometry.left, geometry.top, width, height);
    println!("Saving result to {}...", outfile);

    let cropped = original.crop_imm(geometry.left, geometry.top, width, height);

    cropped.save(outfile).expect("Failed to save to OUTFILE")
}

fn rotate(infile: String, outfile: String, angle: u16) {
    println!("Rotating {} {} degrees...\nSaving result to: {}...", infile, angle, outfile);

    let original = open_image(infile);

    let rotated = match angle {
        90 => original.rotate90(),
        180 => original.rotate180(),
        270 => original.rotate270(),
        _ => panic!("Not a valid angle: {}", angle)
    };

    save_image(rotated, outfile);
}

fn invert(infile: String, outfile: String) {
    println!("Inverting {}'s colors...\nSaving resulto to {}...", infile, outfile);

    let mut image = open_image(infile);
    image.invert();
    save_image(image, outfile);
}

fn grayscale(infile: String, outfile: String) {
   println!("Converting {} to grayscale...\nSaving result to {}...", infile, outfile);

   let original = open_image(infile);
   let grayscale = original.grayscale();
   save_image(grayscale, outfile);
}

fn generate(outfile: String, width: u32, height: u32, color: [u8; 3], noise: bool) {
    let noisy = if noise { "noisy " } else { "" };
    println!("Generating a {}{} by {} pixels image with color {:?}...", noisy, width, height, color);
    println!("Saving output to {}...", outfile);

    let mut buffer = image::RgbImage::new(width, height);

    let (mut lbound, mut ubound): (i16, i16) = (0, 0);
    let mut rng = rand::thread_rng();
    let mut pixel_color = color.clone();

    if noise {
        let max = *pixel_color.iter().max().unwrap();
        let min = *pixel_color.iter().min().unwrap();
        ubound = 255 - (max as i16);
        lbound = 0 - (min as i16);
    }

    for x in 0..width {
        for y in 0..height {
            if noise {
                pixel_color = color.clone();
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

    buffer.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
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

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
