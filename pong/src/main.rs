use ggez::{conf, ContextBuilder, event};
use pong::game::gamestate;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "joaonsantos")
    .window_setup(conf::WindowSetup::default().title("Pong"))
    .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
    .build()
    .expect("failed to create context");

    let main_state = gamestate::GameState::new(&mut ctx);
    event::run(ctx, event_loop, main_state)
}