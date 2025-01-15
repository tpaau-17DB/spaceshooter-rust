extern crate ncurses;
use ncurses::*;

use std::thread;
use std::time::Duration;

mod utils;
use crate::utils::{
    drawing::*,
    movement::*,
    vectors::*,
    enemies::*,
    scenes::*,
    banners::*,
    player::*,
};

fn main() 
{
    // this must be the first thing called when the program starts
    setlocale(LcCategory::all, "en_US.UTF-8");

    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    nodelay(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // 33 ms sleeptime means that game is running at ~30 FPS.
    // this value is also assumed to be the delta time
    let sleeptime_ms = 33;

    // this value increments every frame
    let mut tick = 0;

    // those are the dimensions of the in-game border
    let gamewindow_x = 32;
    let gamewindow_y = 32;

    let win_offset_x = 7;
    let win_offset_y = 12;

    let mut scene: Scene = Scene::Game;

    let mut asteroids: Vec<BasicEnemy> = Vec::new();

    let bounds: Vector = Vector{x: gamewindow_x / 2, y: gamewindow_y / 2};

    let mut player = Player::default();
    player.position = Vector{x: 0, y: gamewindow_y / 2};
    player.set_bounds(&bounds);

    let mut win_x: i32;
    let mut win_y: i32;
    let mut win_dimensions: Vector;

    // the main program loop, executed until 'q' is received
    loop 
    {
        win_x = 0;
        win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);
        win_dimensions = Vector{x: win_x, y: win_y};

        // display the message saying that the window is too small
        if win_x < (gamewindow_x + win_offset_x) as i32
        || win_y < (gamewindow_y + win_offset_y) as i32 {
            clear();

            let message = String::from("The terminal window is too small.");
            draw_message_center(&message, &win_x, &win_y);

            let input = getch();
            match input 
            {
                val if val == ('q' as i32) => break,
                _ => {}
            }

            thread::sleep(Duration::from_millis(100));
        }


        // user input
        let input = getch();
        match input 
        {
            val if val == ('q' as i32) => break,
            _ => 
            {
                match input as i32 {
                    KEY_UP => player.position.y -= 2,
                    KEY_DOWN => player.position.y += 2,
                    KEY_LEFT => player.position.x -= 3,
                    KEY_RIGHT => player.position.x += 3,
                    _ => {},
                };
            }
        }


        // do specific actions depending on the current scene
        match scene
        {
            Scene::MainMenu =>
            {
                panic!("Scene not yet implemented!");
            }

            Scene::Game => 
            {
                player.tick(&asteroids);
                player.score += update_enemies(&mut asteroids, &bounds,
                    &mut player, &tick);

                // make sure player is in bounds
                force_bounds_player(&mut player, &bounds);

                if player.dead()
                {
                    // game over!
                    scene = Scene::GameOver;
                }

                // draw game objects
                clear();
                draw_asteroids(&asteroids, &win_dimensions);
                draw_player(&player, &win_dimensions);
                draw_player_bullets(&player.bullets(), &win_dimensions);

                draw_outline(win_x / 2 - bounds.x as i32 - 3, win_y / 2 - bounds.y as i32 - 1, 2 * bounds.x as i32 + 7, 2 * bounds.y as i32 + 6);
            }

            Scene::GameOver => 
            {
                draw_banner_center(&GAME_OVER_BANNER, &VEC_ZERO, &win_dimensions);
                thread::sleep(Duration::from_millis(100));
            }
        }


        // display score
        let message = format!("Score: {}", player.score);
        mv(win_dimensions.y / 2 + bounds.y + win_offset_y / 2 - 1, win_dimensions.x / 2 - bounds.x - win_offset_x / 2);
        addstr(&message);


        // draw debug overlay if needed
        #[cfg(feature = "debug")]
        {
            let mut message = format!("Player position: x: {}, y: {}",
            player.position.x, player.position.y);
            mv(0, 0);
            addstr(&message);

            message = format!("Bounds: x: {}, y: {}", bounds.x, bounds.y);
            mv(1, 0);
            addstr(&message);

            message = format!("Window size: x: {}, y: {}", win_x, win_y);
            mv(2, 0);
            addstr(&message);

            message = format!("Bullets: {}", player.bullets().len());
            mv(3, 0);
            addstr(&message);

            message = format!("Asteroids: {}", asteroids.len());
            mv(4, 0);
            addstr(&message);

            message = format!("Tick: {}", tick);
            mv(5, 0);
            addstr(&message);

            message = format!("Target FPS: {}", 1000.0 * (1.0 / sleeptime_ms as f32));
            mv(6, 0);
            addstr(&message);
        }

        tick += 1;
        refresh();

        thread::sleep(Duration::from_millis(sleeptime_ms as u64));
    }

    endwin();
}
