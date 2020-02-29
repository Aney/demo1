
/// GGEZ uses radians for rotation
/// This converts normal degrees to radians
pub fn deg_to_rotate(degree: i16) -> f32{
  // 3.1416 radians = 180 deg
  // 1 radian = 57.2958
  // 1 Degree in radians = 0.0174...
  degree as f32 * 0.01745333333
}

