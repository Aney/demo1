use ggez::Context;
use ggez::graphics;
use ggez::nalgebra;
use ggez::GameResult;

pub trait Movement{
  fn move_character(&mut self);
  fn move_x(&mut self, movement: f32);
  fn move_y(&mut self, movement: f32);
}

impl Movement for Character
  {
  fn move_character(&mut self){
    if self.position.up{
      self.move_y(-self.stats.speed);
    }
    if self.position.down{
      self.move_y(self.stats.speed);
    }
    if self.position.left{
      self.move_x(-self.stats.speed);
    }
    if self.position.right{
      self.move_x(self.stats.speed);
    }
  }

  fn move_y(&mut self, movement: f32){
    self.position.y += movement;
  }

  fn move_x(&mut self, movement: f32){
    self.position.x += movement;
  }
}

/// A standard character in the game.
/// Player or NPC
pub struct Character{
  name: String,
  xp: u64,
  pub position: Position,
  stats: Statistics,
}

pub struct Statistics{
  speed: f32,
  attack: u8,
  max_health: u8,
  current_health: u8,
}

pub struct Position{
  pub x: f32, // Public for now, will change to tuple "get_position"
  pub y: f32,
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
}

impl Character{
  /// Creates a new default character
  /// Params: Name &str
  pub fn new(ctx: &Context, name: &str) -> Character{
    let character:Character = Character{
      name: name.to_string(),
      xp: 0,
      position: Position{
        x: 10.0,
        y: 300.0,
        up: false,
        down: false,
        left: false,
        right: false,
      },
      stats: Statistics{
        speed: 2.0,
        attack: 5,
        max_health: 10,
        current_health: 10,
      },
    };

    // Return
    character
  }

  pub fn change_speed(&mut self, speed: f32){
    self.stats.speed += speed;
  }

  pub fn draw_character(&mut self, ctx: &mut Context) -> GameResult{
    // Mesh
    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      nalgebra::Point2::new(0.0, 0.0),
      25.0,  // Radius
      0.01,  // Tolerance
      graphics::WHITE,
    )?;

    graphics::draw(ctx,
      &circle,
      (nalgebra::Point2::new(
          self.position.x,
          self.position.y),
      )
    )
  }
}