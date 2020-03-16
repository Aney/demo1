use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::event::{KeyCode, KeyMods};

mod character;
use character::{Character};

mod utilities;
use utilities::{WINDOW_HEIGHT, WINDOW_WIDTH};

mod enums;
use enums::AttackPosition;

struct MyGame {
    player: Character,  // Player's toon
    npc: Vec<Character>,
    projectiles: Vec<Character>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut x = MyGame {
            player: character::new_character("Test"),  // Player's Toon 
            npc: Vec::new(),
            projectiles: Vec::new(),
        };

        x.npc.push(character::new_character("Enemy"));

        x
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        for x in 0..self.projectiles.len() {
            self.projectiles[x].position.x += 2.0;   
        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context,
        keycode: KeyCode, _keymods: KeyMods, _repeat: bool) 
    {
        if keycode == KeyCode::Escape{ ggez::event::quit(ctx) }
        
        match keycode{
            KeyCode::Escape => ggez::event::quit(ctx),
            KeyCode::W => character::move_y(&mut self.player.position, -5.0),
            KeyCode::S => character::move_y(&mut self.player.position, 5.0),
            KeyCode::A => character::move_x(&mut self.player.position, -5.0),
            KeyCode::D => character::move_x(&mut self.player.position, 5.0),
            KeyCode::Right => character::shoot(&self.player.position.x,
                &self.player.position.y,
                &self.player.direction,
                &mut self.projectiles),
            _ => println!("You pressed: {:?}", keycode)
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context,
        keycode: KeyCode, _keymods: KeyMods) 
    {
        match keycode{
            _ => println!("You released: {:?}", keycode)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE); // Clears to white
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into()); // clear's to colour

        character::draw_entity(
            ctx, &self.player.position)?;

        for entity in self.npc.iter(){
            character::draw_entity(ctx, &entity.position)?;
        }

        for projectile in self.projectiles.iter(){
            character::draw_entity(ctx, &projectile.position)?;
        }

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
