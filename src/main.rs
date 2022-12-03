use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
// mod rect;
// pub use rect::Rect;

pub struct State {
    pub ecs: World,
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
        // tells Specs that if any changes were queued up by the systems,
        // they should apply to the world now.
        self.ecs.maintain();
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

    rltk::main_loop(context, gs)
}
