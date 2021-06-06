use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

#[derive(Component)] // derive the Component boilerplate
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

// ! #[derive(Component)]
// ! struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}

#[derive(PartialEq, Copy, Clone)]
// Clone allows a copy to be made programmatically
// Copy changes the default from moving the object to copying on assignment
enum TileType {
    Wall, Floor
}

struct State {
    ecs: World
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
    // multiply the y position by the map width (80) then add x
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50]; // (map is 80 * 50)

    // Make the borders walls
    for x in 0..80 {
        map[xy_idx(x,0)] = TileType::Wall; // GET the vector index for (x, 0) then SET it to be a wall
        map[xy_idx(x,49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Randomly place a bunch of walls

    let mut rng = rltk::RandomNumberGenerator::new();    
    for _i in 0..400 { // don't care about the value of i, just want to go 400 times
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y); // the random location this iteration
        if idx != xy_idx(40, 25) { // ensure that the wall is not created on top of the player!
            map[idx] = TileType::Wall;
        }
    }

    map

}


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) { // deltas are how much the player should move by
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    // gain write access to Position and Player
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall { // can move as long as it isnt a wall
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49 , max(0, pos.y + delta_y));
        }
    }
    // Join the two, adding delta_x to x and delta_y to y
    // Also ensure that the player can't leave the screen!
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {} // anything else we ignore
        },
    }
}

fn draw_map(map: &[TileType], ctx : &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl GameState for State { // State structure implements the trait GameState
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        // obtain Read access to the container storing Position components
        let renderables = self.ecs.read_storage::<Renderable>();
        // ask for read access to the Renderable storage
        // (Where to draw and what to draw!)
            
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
        // join positions and renderables, only returning entities w/ both

    
        
    }
}

// ! struct LeftWalker {}

/*impl<'a> System<'a> for LeftWalker { // 'a are "lifetime" specifiers (The components it uses must exist long enough for the system to run)
    type SystemData = (ReadStorage<'a, LeftMover>,
                       WriteStorage<'a, Position>);
    // system needs to SEE leftmover components, but ALTER position components

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join() { // the "_" tells rust we know we don't use this, its not an error!
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }

}*/

impl State {
    fn run_systems(&mut self) { // can access the data in its instance of State w/ the "self" keyword
        // ! let mut lw = LeftWalker{}; // make a new alterable instance of the LeftWalker system
        // ! lw.run_now(&self.ecs); // run using the self ecs
        self.ecs.maintain(); // if any changes were queued by the systems, then apply them to the world now
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // ! gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.insert(new_map()); // makes map available from anywhere the ECS can access

    gs.ecs // create the player entity
        .create_entity() // returns a new empty entity
        .with(Position { x: 40, y: 25 }) // attach a position to the entity
        .with(Renderable { // attach the renderable component
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build(); // takes the assembled entity and puts together the disparate parts

    /* for i in 0..10 { // create 10 entities (0 to 9)
        gs.ecs
        .create_entity()
        .with(Position { x: i * 7, y: 20 })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(LeftMover{})
        .build();
    } */

    rltk::main_loop(context, gs)
}