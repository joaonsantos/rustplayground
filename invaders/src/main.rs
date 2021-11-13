use std::{error::Error, io, sync::mpsc, thread, time::{Duration, Instant}};

use crossterm::{ExecutableCommand, cursor::{Hide, Show}, event::{self, Event}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use invaders::{frame::{self, Drawable, Frame, new_frame}, invaders::Invaders, player::{Player}, render};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("start", "./sounds/startup.mp3");
    audio.add("quit", "./sounds/quit.mp3");
    audio.add("pew", "./sounds/pew.mp3");
    audio.add("explosion", "./sounds/explosion.mp3");
    audio.add("win", "./sounds/win.mp3");
    audio.add("lose", "./sounds/lose.mp3");
    audio.play("start");

    // terminal setup
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel::<Frame>();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut invaders = Invaders::new();
    let mut instant = Instant::now();
    'gameloop: loop {
        // per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        if event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Left => {
                        player.move_left();
                    }
                    event::KeyCode::Right => {
                        player.move_right();
                    }
                    event::KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    event::KeyCode::Esc => {
                        audio.play("quit");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        invaders.update(delta);
        if player.detect_hits(&mut invaders) {
            audio.play("explosion");
        }

        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }

        // draw & render
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(16));
    }

    // cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
