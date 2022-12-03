use rltk::{Rltk, GameState};

struct State {}
impl GameState for State {
    // Tick with each frame rendered
    fn tick(&mut self, ctx : &mut Rltk) {
        // cls = clear the screen
        ctx.cls();
        // Print output at x, y coords
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}