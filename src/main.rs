use rltk::{Rltk, GameState}; // Obtain Rltk and GameState from the rltk namespace

struct State {} 
impl GameState for State { // State structure implements the trait GameState
    fn tick(&mut self, ctx : &mut Rltk) {c
        ctx.cls();
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