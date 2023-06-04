use robo_halloween::{
    audio::Audio,
    frame::{self, Drawable},
    player, pumpkin, render, score,
    stopwatch::Stopwatch,
    zombie,
};
use std::{error::Error, io::stdout, sync::mpsc, thread, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Music setup for the game
    let mut music = Audio::new();
    music.upload_audios();
    music.play("game_music");
    let game_music_player = music.get_sink("game_music");

    // Create blank terminal canvas
    terminal::enable_raw_mode()?;

    let mut output = stdout();
    output.execute(EnterAlternateScreen)?;
    output.execute(Hide)?;

    // Transmitter and Receiver for frames in game loop
    let (t_ch, r_ch) = mpsc::channel();

    let render_thread = thread::spawn(move || {
        let first_frame = frame::frame();
        let mut output_render = stdout();

        render::render(&first_frame, &first_frame, &mut output_render);

        let mut prev_frame = first_frame;

        // Keep receiving and rendering new frames
        loop {
            let curr_frame = match r_ch.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };
            render::render(&curr_frame, &prev_frame, &mut output_render);
            prev_frame = curr_frame;
        }
    });

    // Initialize entities
    let mut robot = player::Robot::new();
    let mut pumpkins = pumpkin::Pumpkins::new();
    let mut zombies = zombie::Zombies::new();
    let mut score_board = score::Score::new();
    let stopwatch = Stopwatch::new(90);

    // Game logic
    'game_running: loop {
        // Starts render_thread and also stops from sending multiple frames
        thread::sleep(Duration::from_millis(1));

        if let Some(music_player) = game_music_player {
            if music_player.empty() {
                music.play("game_music");
            }
        }

        while event::poll(Duration::default())? {
            if let Event::Key(key_press) = event::read()? {
                match key_press.code {
                    KeyCode::Up => robot.move_up(),
                    KeyCode::Down => robot.move_down(),
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if robot.shoot() {
                            music.play("shoot");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => break 'game_running,
                    _ => {}
                }
            }
        }

        // Perform updates to the entities
        if robot.is_alive() {
            robot.update();

            if robot.smash_pumpkins(&mut pumpkins) {
                music.play("hit_pumpkin");
                score_board.add_pumpkin();
            };
            pumpkins.update();

            if robot.kill_zombies(&mut zombies) {
                music.play("hit_zombie");
                score_board.add_zombie();
            };
            zombies.eat_pumpkins(&mut pumpkins);
            zombies.update();

            if zombies.reached_end() {
                music.play("player_dead");
                robot.kill_player("☠️");
            }
            if stopwatch.expired() {
                music.play("finished_game");
                robot.kill_player("🚩");
            }
            score_board.update_time_left(stopwatch.time_remaining());
        }

        // Keep sending new frames to render thread
        let mut new_frame = frame::frame();
        zombies.draw(&mut new_frame);
        robot.draw(&mut new_frame);
        pumpkins.draw(&mut new_frame);
        score_board.draw(&mut new_frame);

        t_ch.send(new_frame).unwrap();
    }

    // Exit and clean render thread
    drop(t_ch);
    render_thread.join().unwrap();

    // Close terminal canvas
    output.execute(LeaveAlternateScreen)?;
    output.execute(Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
