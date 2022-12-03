use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
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
        let mut lw = LeftWalker{};

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

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
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
        }
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
    type SystemData = (ReadStorage<'a, LeftMover>, 
                        WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        // underscore before the LeftMover variable name: we never
        // actually use it, we just require that the entity has one. 
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
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

    // This is the Player
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();    

    // These are the "Others"
    for i in 0..10 {
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
    }
    
    rltk::main_loop(context, gs)
}
