use image::imageops::FilterType;
use image::GenericImageView;

fn main() {
    let default_img = image::open("./rust-logo.png").unwrap();
    let aspect_ratio = (default_img.width() / default_img.height()) as f32;
    let resize_height = (50.0 * aspect_ratio) as u32;
    let resize_width = 50;
    let img = default_img.resize(resize_width, resize_height, FilterType::Nearest);

    let density_from_light_to_dark = vec!['*', '=', '+', '~', '-', ',', '.', ' '];

    for height in 0..resize_height {
        for width in 0..resize_width {
            let rgba = img.get_pixel(width, height);
            let r = rgba[0];
            // TODO: check other value for green, blue
            // let g = rgba[1];
            // let b = rgba[2];

            let idx: usize = (r % 8) as usize;
            print!("{}", density_from_light_to_dark[idx]);
        }

        println!()
    }
}
