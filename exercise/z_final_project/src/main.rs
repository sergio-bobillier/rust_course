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

mod geometry;
mod processing_commands;
mod command_factories;

use image::DynamicImage;
use std::collections::VecDeque;
use std::env;
use std::process::exit;

use processing_commands::ProcessingCommand;
use command_factories::CommandsFactory;

const ERR_NOT_ENOUGH_ARGUMENTS:i32 = 1;
const ERR_COMMAND_PARSING_ERROR:i32 = 2;

fn print_usage() {
    println!("Usage\n");

    println!("  mirage INFILE [COMMAND_1 [OPTIONS]] ... [COMMAND_N [OPTIONS]] OUTFILE\n");

    println!("Available commands:\n");

    println!("  • --blur AMOUNT");
    println!("  • --crop LEFT TOP [WIDTH] [HEIGHT]");
    println!("  • --brighten DELTA");
    println!("  • --rotate ANGLE");
    println!("  • --invert");
    println!("  • --grayscale");

    println!("\nNote: - can be used as placeholder for optional parameters\n");
}

fn print_error(message: String, exit_code: i32) {
    println!("Error: {}\n", message);
    print_usage();
    exit(exit_code);
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
    // * Fractal

    let mut args: VecDeque<String> = env::args().collect();

    if args.len() < 3 {
        print_error(
            "Error: Not enough arguments".to_string(),
            ERR_NOT_ENOUGH_ARGUMENTS
        )
    }

    // Pop the first element of the vector (the name of the executable).
    args.pop_front();

    let input_file = args.pop_front().unwrap();
    let output_file = args.pop_back().unwrap();

    let result = CommandsFactory::new(args).create();

    match result {
        Err(message) => {
            print_error(message, ERR_COMMAND_PARSING_ERROR)
        },
        Ok(commands) => {
            let mut image = open_image(&input_file);

            for mut command in commands {
                print!("Applying {}... ", command.name());
                command.pre_process(&image);
                image = command.run(image);
                println!("Ok {}", command.description());
            }

            save_image(image, &output_file);
        }
    }
}

fn open_image(path: &String) -> DynamicImage {
    println!("Reading {}...", path);
    let message = format!("Failed to open: {}", path);
    image::open(path).expect(&message)
}

fn save_image(image: DynamicImage, path: &String) {
    println!("Saving result to {}...", path);
    let message = format!("Failed to save to: {}", path);
    image.save(path).expect(&message);
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
