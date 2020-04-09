use crate::color::Color;

use std::fs::{create_dir_all, File};
// use std::io::{self, BufRead, Write, BufReader};
// use std::fmt;
use std::io::Write;
use chrono::Local;

pub struct Image {
  // data: pixel color Vec[y][x]
  pub data: Vec<Vec<Color>>
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      data: vec![vec![Color(0, 0, 0); width]; height]
    }
  }

  pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
    self.data[y][x] = color;
  }

  pub fn output(&self) -> std::io::Result<()> {
    let dir = "output/";
    create_dir_all(dir)?;
    let filename = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let mut file = File::create(format!("{}{}.ppm", dir, filename))?;

    let size = self.get_size();
    let mut ppm = String::new();
    ppm.push_str("P3\n");
    ppm.push_str(&format!("{} {}\n", size.0, size.1));
    ppm.push_str("255\n");
    ppm.push_str(&self.stringify());
    ppm.push_str("0 0 0\n");

    file.write_all(ppm.as_bytes()).unwrap();
    println!("{}{}.ppm", dir, filename);
    Ok(())
  }

  pub fn get_size(&self) -> (usize, usize) {
    let h = self.data.len();
    if h == 0 {
      return (0, 0)
    }
    let w = self.data[0].len();
    (w, h)
  }

  fn stringify(&self) -> String {
    let mut ret = String::new();
    for row in &self.data {
      for color in row {
        ret.push_str(&color.to_ppm());
        ret.push_str("\n");
      }
    }
    ret
  }
}
