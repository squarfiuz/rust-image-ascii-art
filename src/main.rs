use image::{GenericImageView};

fn get_str_ascii(intent: u8) -> &'static str  {
    let index = intent/29;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "#", "@"];
    return ascii[index as usize];
}

fn ascii_image(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let intent = if pix[3] == 0 { 0 } else { pix[0] / 3 + pix[1] / 3 + pix[2] / 3 };

                print!("{}", get_str_ascii(intent));
            }
        }

        if y % (scale * 2) == 0 {
            println!("");
        }
    }
}

fn main() {
    ascii_image("assets/monkey.png", 4);
}