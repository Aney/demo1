use ggez::Context;
use ggez::graphics;
use ggez::nalgebra;
use ggez::GameResult;

// SoA Structure of Arrays
struct Entity{
    id:i32,
    components: Vec<Component>
}

struct Component{

}

enum Entity_Type{
    Player, // Green
    NPC,    // Blue
    Enemy,  // Red
    Gold,   // Yellow
    Item,   // Purple
    Attack, // Black
}

// Physical have size and position

struct _Size{
    width: u8,
    height: u8
}

// Components should be pure data structures (Data)

// Systems contain the behaviours (Behaviour)

pub struct Position{
    pub x: f32, // Public for now, will change to tuple "get_position"
    pub y: f32,
}

pub struct Direction{
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

struct _Health{
    max_health: u8,
    health: u8,
}

pub struct _Statistics{
    speed: f32,
    attack: u8,
}

struct _Levelling{
    level: u8,
    max_xp: u8,
    xp: u8,
}

// Player character
pub struct Character{
    _name: String,
    pub position: Position,
    entity: Entity_Type,
    pub direction: Direction,
}

trait Controllable{

}

/// Creates a new default character
/// Params: Name &str
pub fn new_character(name: &str) -> Character{
    Character{
        _name: name.to_string(),
        position: Position{
            x: 10.0,
            y: 10.0
        },
        direction: Direction{
            up: false,
            down: false,
            left: false,
            right: false,
        },
        entity: Entity_Type::Player
    }
}

pub fn shoot(x: &f32, y: &f32, direction: &Direction, vect: &mut Vec<Character>){
    vect.push(new_attack(x, y, direction));
}

pub fn new_attack(x: &f32, y: &f32, direction: &Direction) -> Character{
    Character{
        _name: "Chef".to_string(),
        position: Position{
            x: *x,
            y: *y,
        },
        direction: Direction{
            up: false,
            down: false,
            left: false,
            right: false,
        },
        entity: Entity_Type::Attack
    }
}

pub fn move_entity(position: &mut Position, x:f32, y:f32){
    position.x += x;
    position.y += y;
}

pub fn move_y(position: &mut Position, y:f32){
    position.y += y
}

pub fn move_x(position: &mut Position, x:f32){
    position.x += x
}

pub fn draw_entity(ctx: &mut Context, position: &Position) -> GameResult{
    // Mesh
    let circle = graphics::Mesh::new_rectangle(
        ctx, 
        graphics::DrawMode::fill(), 
        graphics::Rect::new_i32(0, 0, 32, 32),
        graphics::WHITE
    )?;

    graphics::draw(ctx,
        &circle,
        (nalgebra::Point2::new(
            position.x,
            position.y),
        )
    )
}