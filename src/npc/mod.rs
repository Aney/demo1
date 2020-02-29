use ggez::Context;
use ggez::graphics;
use ggez::nalgebra;
use ggez::GameResult;

pub struct NPC{
  name: String,
  xywh: [f32; 4],
}

impl NPC{
  pub fn new(x:f32, y:f32, w:f32, h:f32) -> NPC{
    NPC{
      name: "yegg".to_string(),
      xywh: [x,y,w,h],
    }
  }

  pub fn draw_entitiy(&mut self, ctx: &mut Context) -> GameResult{
    // Square
    let square = graphics::Mesh::new_rectangle(
      ctx,
      graphics::DrawMode::fill(),
      graphics::Rect::new_i32(10, 15, self.xywh[2] as i32, self.xywh[3] as i32),
      [0.6, 0.2, 0.2, 1.0].into()
    )?;

    graphics::draw(ctx,
      &square,
      (nalgebra::Point2::new(
          self.xywh[0],
          self.xywh[1]),
      )
    )
  }
}