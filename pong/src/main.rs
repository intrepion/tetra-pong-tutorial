use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

struct GameState {}

impl State for GameState {}

fn main() {
    ContextBuilder::new("Pong", 640, 480)
        .quit_on_escape(true)
        .build();
}
