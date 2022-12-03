use rltk::{GameState, Rltk, Tile, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

struct State {
    ecs: World,
}

impl GameState for State {
    // Tick with each frame rendered
    fn tick(&mut self, ctx: &mut Rltk) {
        // cls = clear the screen
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        // Print output at x, y coords
        // ctx.print(1, 1, "Hello Rust World");
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// Means we want to implement funcionality to State
impl State {
    // again &mut self means must be allowed to change things,
    // means it can access data in its instance of State
    // with the Self keyword
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};

        // tells the system to run, and tells it how to find the ECS.
        lw.run_now(&self.ecs);

        // tells Specs that if any changes were queued up by the systems,
        // they should apply to the world now.
        self.ecs.maintain();
    }
}

// #[derive(Component)] is a macro that says "from my basic data, please derive the boilerplate needed for x";
// in this case, the x is a Component.
#[derive(Component)]
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

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

// This function takes the current game state and context,
// looks at the key variable in the context, and calls the appropriate
// move command if the relevant movement key is pressed.
fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // match "unwraps" value to get data out of Option
    match ctx.key {
        // Option types have two possible value: None (no data),
        // or Some(x) - indicating that there is data here, held inside.
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

// a component with no data like this is called a "tag component".
#[derive(Component)]
struct LeftMover {}

// defines an empty structure - somewhere to attach the logic.
struct LeftWalker {}

// we are implementing Specs' System trait for our LeftWalker structure.
// The 'a are lifetime specifiers: the system is saying that the components
// it uses must exist long enough for the system to run.
impl<'a> System<'a> for LeftWalker {
    // type to tell Specs what the system requires
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        // underscore before the LeftMover variable name: we never
        // actually use it, we just require that the entity has one.
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

// Any function that ends with a statement that lack
// a semicolon treats that line as a return statement.
// ---
// multiplies the y position by the map width (80), and adds x.
// This guarantees one tile per location, and efficiently maps it
// in memory for left-to-right reading.
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    // The first parameter is the value for each element of the new vector. In this case, we're setting every entry we create to be a Floor (from the TileType enumeration).
    // The second parameter is how many tiles we should create. They will all be set to the value we set above.
    // In this case, our map is 80x50 tiles (4,000 tiles - but we'll let the compiler do the math for us!). So we need to make 4,000 tiles.
    let mut map = vec![TileType::Floor; 80 * 50];

    // Make boundaries Walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Randomnly Splat Walls
    // First, obtain Thread-local RNG
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);

        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall
        }
    }

    // returns Map
    map
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        // Render a tile depending on tile type
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        // move the coords
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Notice that World::new() is another constructor - it's a method inside the World type,
    // but without a reference to self.
    // So it doesn't work on existing World objects - it can only make new ones.
    let mut gs = State { ecs: World::new() };

    // What this does is it tells our World to take a look at the types we are giving it,
    // and do some internal magic to create storage systems for each of them.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    // This is the Player
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    // These are the "Others"
    // Not used in ch 2.2
    // for i in 0..10 {
    //     gs.ecs
    //     .create_entity()
    //     .with(Position { x: i * 7, y: 20 })
    //     .with(Renderable {
    //         glyph: rltk::to_cp437('☺'),
    //         fg: RGB::named(rltk::RED),
    //         bg: RGB::named(rltk::BLACK),
    //     })
    //     .with(LeftMover{})
    //     .build();
    // }

    rltk::main_loop(context, gs)
}
