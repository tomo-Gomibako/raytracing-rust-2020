pub struct Color(pub u8, pub u8, pub u8);

impl Color {
  pub fn to_ppm(&self) -> String {
    format!("{} {} {}", self.0, self.1, self.2)
  }
}

impl Clone for Color {
  fn clone(&self) -> Self {
    Color(self.0, self.1, self.2)
  }
}
