use ggez::Context;
use ggez::graphics;
use ggez::nalgebra;
use ggez::GameResult;

// Things are public, but will be altered where need be, just for PoC

// SoA Structure of Arrays
struct Entity{
    id:i32,
    components: Vec<Component>
}

struct Component{

}

pub enum Entity_Type{
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

#[derive(Copy, Clone)]
pub struct Direction{
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

struct Health{
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
    pub entity: Entity_Type,
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
        entity: Entity_Type::Player,
    }
}

pub fn shoot(x: &f32, y: &f32, direction: &Direction, vect: &mut Vec<Character>){
    vect.push(new_attack(x, y, direction));
}

pub fn new_attack(x: &f32, y: &f32, direction: &Direction) -> Character{
    let x:Character = Character{
        _name: "Chef".to_string(),
        position: Position{
            x: *x,
            y: *y,
        },
        direction: *direction,
        entity: Entity_Type::Attack,
    };

    x
}

pub fn move_entity(character: &mut Character){
    let mut temp_speed:f32 = 0.0;

    match character.entity{
        Entity_Type::Player => temp_speed = 2.0,
        Entity_Type::Attack => temp_speed = 5.0,
        _ => temp_speed = 3.0,
    }
    

    if character.direction.up{
        character.position.y -= temp_speed;
    }
    if character.direction.down{
        character.position.y += temp_speed;
    }
    if character.direction.left{
        character.position.x -= temp_speed;
    }
    if character.direction.right{
        character.position.x += temp_speed;
    }
}

pub fn move_y(position: &mut Position, y:f32){
    position.y += y
}

pub fn move_x(position: &mut Position, x:f32){
    position.x += x
}

pub fn draw_entity(ctx: &mut Context, position: &Position,
    entity: &Entity_Type) -> GameResult{

    let mut size:i32 = 0;

    match entity{
        Entity_Type::Player => size = 32,
        Entity_Type::Attack => size = 16,
        _ => size = 0,
    }
    // Mesh
    let circle = graphics::Mesh::new_rectangle(
        ctx, 
        graphics::DrawMode::fill(), 
        graphics::Rect::new_i32(0, 0, size, size),
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