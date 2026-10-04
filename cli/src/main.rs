mod read_image;

use inference::infer;
use crate::read_image::*;

fn main() {
    // Require filename as first CLI arg; print usage and exit if missing
    let mut args = std::env::args();
    let exe = args.next().unwrap_or_else(|| "inference".to_string());
    let filename = match args.next() {
        Some(f) => f,
        None => {
            eprintln!("Usage: {} <input-file> [image_index]", exe);
            std::process::exit(1);
        }
    };
    let image_index = args.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let (plane0, plane1) = read_image(&filename, image_index).unwrap();

    let ret = infer(&plane0, &plane1);

    println!("Inference result: class = {}, scores = {:?}, early = {}", ret.class, ret.scores, ret.early);
}
