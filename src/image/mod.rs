mod image;

use crate::color::Color;

pub fn output(data: Vec<Vec<Color>>) {
  let image = image::Image {
    data: data
  };
  image.output();
}
