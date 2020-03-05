use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;

mod character;
use character::{Character, Movement};

mod utilities;
use utilities::{WINDOW_HEIGHT, WINDOW_WIDTH};

mod npc;
use npc::NPC;

mod enums;
use enums::AttackPosition;

struct MyGame {
    player: Character,  // Player's toon
    attacking: bool,
    attack_position: AttackPosition,
    rotation: f32,
    evil_fellas: Vec<NPC>,
    attack_pos: [f32; 2],
    movers: Vec<character::Movement>
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut x = MyGame {
            player: Character::new(ctx, "Test"),  // Player's Toon 
            attacking: false,
            attack_position: AttackPosition::NONE,
            rotation: 0.0,
            evil_fellas: Vec::new(),
            attack_pos: [0.0,0.0],
        };

        // 
        for oo in 1..=10{
            x.add_evil(oo as f32*32.0, oo as f32*37.0, 30.0, 30.0);
        }
        // x.add_evil(10.0,10.0, 50.0, 50.0);
        // x.add_evil(70.0,10.0, 50.0, 50.0);
        // x.add_evil(150.0,10.0, 50.0, 50.0);

        x
    }

    /// Create a new evil dude. Currently an NPC for testing
    pub fn add_evil(&mut self, evil_x:f32, evil_y:f32, evil_w:f32, evil_h:f32){
        self.evil_fellas.push(
            npc::NPC::new(evil_x, evil_y, evil_w, evil_h)
        );
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.player.move_character();

        // Change attack position
        if self.attacking{
            // TEMPORAY FOR TESTING
            match self.attack_position{
                AttackPosition::DOWN => self.attack_pos[1] = 25.0,
                AttackPosition::UP => {self.attack_pos[1] = -25.0;
                    self.attack_pos[0] = 25.0;},
                AttackPosition::LEFT => {self.attack_pos[0] = -25.0;
                    self.attack_pos[1] = -25.0;},
                AttackPosition::RIGHT => {self.attack_pos[0] = 25.0;
                    self.attack_pos[1] = 25.0;},
                _ => {}//print!("sfa", )
            }
        }

        // If the player x/y
        // overlaps one of the evil_fellas
        for evil in self.evil_fellas.iter(){
            println!("Attack: {} {}",
                self.player.position.x + self.attack_pos[0],
                self.player.position.y + self.attack_pos[1]);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode{
            KeyCode::Escape => ggez::event::quit(ctx),
            KeyCode::W => self.player.position.up = true,
            KeyCode::S => self.player.position.down = true,
            KeyCode::A => self.player.position.left = true,
            KeyCode::D => self.player.position.right = true,
            KeyCode::Up => {
                self.attack_position = AttackPosition::UP;
                self.attacking = true;
                self.rotation = utilities::deg_to_rotate(180);},
            KeyCode::Down => {
                self.attack_position = AttackPosition::DOWN;
                self.attacking = true;
                self.rotation = utilities::deg_to_rotate(0);},
            KeyCode::Left => {
                self.attack_position = AttackPosition::LEFT;
                self.attacking = true;
                self.rotation = utilities::deg_to_rotate(90);},
            KeyCode::Right => {
                self.attack_position = AttackPosition::RIGHT;
                self.attacking = true;
                self.rotation = utilities::deg_to_rotate(-90);},
            KeyCode::E => self.player.change_speed(0.5),
            KeyCode::Q => self.player.change_speed(-0.5),
            KeyCode::F => character::move_entity(&mut self.player.position,
                [10.0,0.0,0.0,0.0]
            ),
            _ => println!("You pressed: {:?}", keycode)
        }
    }

    fn key_up_event(
        &mut self, 
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods
    ) {
        match keycode{
            KeyCode::W => self.player.position.up = false,
            KeyCode::S => self.player.position.down = false,
            KeyCode::A => self.player.position.left = false,
            KeyCode::D => self.player.position.right = false,
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => 
                {   self.attack_position = AttackPosition::NONE;
                    self.attacking = false;},
            KeyCode::Space => {
                self.rotation += utilities::deg_to_rotate(90);
                println!("Rotation: {}", self.rotation);
            }, //self.attacking = false,
            KeyCode::Return => 
                println!("PosX: {}, PosY: {}",
                    self.player.position.x,
                    self.player.position.y),
            _ => println!("You released: {:?}", keycode)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE); // Clears to white
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into()); // clear's to colour
        // Draw code here...

        self.player.draw_character(ctx)?;

        let attack = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect::new_i32(0, 0, 50, 10),
            graphics::WHITE
        )?;



        if self.attacking{
            graphics::draw(ctx, &attack, 
                graphics::DrawParam::new()
                .dest(na::Point2::new(
                    self.player.position.x + self.attack_pos[0],
                    self.player.position.y + self.attack_pos[1]))
                .rotation(self.rotation)
            )?;
        }

        // self.evil_fellas[0].draw_character(ctx)?;

        for i in 0..self.evil_fellas.len(){
            self.evil_fellas[i].draw_entitiy(ctx)?;
        }

        // for evil in self.evil_fellas.iter(){
        //     // evil.draw_character(ctx);
        // }

        // graphics::draw(ctx, &square, (na::Point2::new(0.0, 300.0),))?;
        // graphics::draw(ctx, &square, (na::Point2::new(300.0, 100.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = 
        ContextBuilder::new(
            "Game",
            "Nathan") // Not 100% what this is for...
        .window_setup(ggez::conf::WindowSetup::default().title("Gamemen Game"))
        .window_mode(ggez::conf::WindowMode::default()
            .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
	    .build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
