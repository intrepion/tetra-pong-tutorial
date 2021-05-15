use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

fn main() {
    ContextBuilder::new("Pong", 640, 480)
        .quit_on_escape(true)
        .build();
}
