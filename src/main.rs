use raytracing::{image::image, color};
use image::Image;
use color::Color;

fn main() {
  let size = (200, 100);
  let mut image = Image::new(size.0, size.1);

  for x in 0..size.0 {
    for y in 0..size.1 {
      let r = (255 * x / size.0) as u8;
      let g = (255 * y / size.1) as u8;
      let b = 255 / 2;
      image.set_color(x, y, Color(r, g, b));
    }
  }

  let _ = image.output();
}
