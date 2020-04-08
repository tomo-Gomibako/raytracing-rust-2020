use raytracing::{image, color};
use color::Color;

fn main() {
  image::output(vec![
    vec![Color(255, 0, 255), Color(0, 255, 0)],
    vec![Color(255, 0, 255), Color(0, 255, 0)],
    vec![Color(255, 0, 255), Color(0, 255, 0)]
  ]);
}
