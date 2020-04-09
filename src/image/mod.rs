pub mod image;

use crate::color::Color;

pub fn output(data: Vec<Vec<Color>>) {
  let image = image::Image {
    data: data
  };
  let _ = image.output();
}
