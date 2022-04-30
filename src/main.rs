use ggez::{conf, event, Context, GameResult};
use specs::{Builder, Component, VecStorage, World, WorldExt}
use std::path;

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update game logic here
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update draw here
        Ok(())
    }
}

pub fn main() -> GameResult {

    pub fn register_components(world: &mut World) {
        world.register::<Position>();
        world.register::<Renderable>();
        world.register::<Player>();
        world.register::<Wall>();
        world.register::<Box>();
        world.register::<BoxSpot>();
    }
    
    pub fn create_wall(world: &mut World, position: Position) {
        world
            .create_entity()
            .with(Position { z: 10, ..position })
            .with(Renderable {
                path: "/images/wall.png".to_string(),
            })
            .with(Wall {})
            .build();
    }
    
    pub fn create_floor(world: &mut World, position: Position) {
        world
            .create_entity()
            .with(Position { z: 5, ..position })
            .with(Renderable {
                path: "/images/floor.png".to_string(),
            })
            .build();
    }
    
    pub fn create_box(world: &mut World, position: Position) {
        world
            .create_entity()
            .with(Position { z: 10, ..position })
            .with(Renderable {
                path: "/images/box.png".to_string(),
            })
            .with(Box {})
            .build();
    }
    
    pub fn create_box_spot(world: &mut World, position: Position) {
        world
            .create_entity()
            .with(Position { z: 9, ..position })
            .with(Renderable {
                path: "/images/box_spot.png".to_string(),
            })
            .with(BoxSpot {})
            .build();
    }
    
    pub fn create_player(world: &mut World, position: Position) {
        world
            .create_entity()
            .with(Position { z: 10, ..position })
            .with(Renderable {
                path: "/images/player.png".to_string(),
            })
            .with(Player {})
            .build();
    }
    

    #[derive(Debug, Component, Clone, Copy)]
    #[storage(VecStorage)]
    pub struct Position {
        x: u8,
        y: u8,
        z: u8,
    }
    
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Renderable {
        path: String,
    }
    
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Wall {}
    
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Player {}
    
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Box {}
    
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct BoxSpot {}

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("touhou_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Touhou Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;
    // Create the game state
    let game = Game {};
    // Run the main event loop
    event::run(context, event_loop, game)
}