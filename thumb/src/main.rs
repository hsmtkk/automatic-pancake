use image::{imageops, GenericImageView};

fn main() {
    let size = 128;
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: thumb in out");
        return;
    }
    let infile = &args[1];
    let outfile = &args[2];
    let mut img = image::open(infile).expect("failed to open input image file");
    let dim = img.dimensions();
    let w = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img2 = imageops::crop(&mut img, (dim.0 - w) / 2, (dim.1 - w) / 2, w, w).to_image();
    let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3);
    img3.save(outfile)
        .expect("failed to save output image file");
}
