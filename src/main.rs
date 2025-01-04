extern crate ncurses;
use ncurses::*;

use std::thread;
use std::time::Duration;

mod utils;

use crate::utils::
{
    drawing::*,
    movement::*,
    vectors::*,
    enemies::*,
    scenes::*,
    banners::*
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

    let debug_overlay = true;

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

    // local Vector [0, 0] represents the center of the screen
    let mut player_position: Vector = Vector{x: 0, y: gamewindow_y / 2};

    let mut player_score = 0;

    let mut player_bullets: Vec<Vector> = Vec::new();
    let bullet_timeout_ms = 200;
    let mut bullet_cooldown = bullet_timeout_ms;

    let mut asteroids: Vec<BasicEnemy> = Vec::new();

    let bounds: Vector = Vector{x: gamewindow_x / 2, y: gamewindow_y / 2};

    let mut win_x: i32;
    let mut win_y: i32;
    let mut win_dimensions: Vector;

    let window_to_small_message = "The terminal window is too small.";

    // the main program loop, executed until 'q' is received
    loop 
    {
        win_x = 0;
        win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);
        win_dimensions = Vector{x: win_x, y: win_y};

        // display the message saying that the window is too small
        if win_x < (gamewindow_x + win_offset_x) as i32 || win_y < (gamewindow_y + win_offset_y) as i32 
        {
            clear();
            let msg = format!("Required size: x: {gamewindow_x}, y: {gamewindow_y}");

            mv(win_y / 2, (win_x - window_to_small_message.len() as i32) / 2);
            addstr(&window_to_small_message);
            mv(win_y / 2 + 1, (win_x - msg.len() as i32) / 2);
            addstr(&msg);

            let input = getch();
            match input 
            {
                val if val == ('q' as i32) => break,
                _ => {}
            }

            thread::sleep(Duration::from_millis(100));
            continue;
        }

        let input = getch();
        match input 
        {
            val if val == ('q' as i32) => break,
            _ => 
            {
                match input as i32 {
                    KEY_UP => player_position.y -= 2,
                    KEY_DOWN => player_position.y += 2,
                    KEY_LEFT => player_position.x -= 3,
                    KEY_RIGHT => player_position.x += 3,
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
                // shoot!
                bullet_cooldown -= sleeptime_ms;
                if bullet_cooldown <= 0
                {
                    bullet_cooldown = bullet_timeout_ms;

                    player_bullets.push(Vector{x: player_position.x, y: player_position.y});
                }

                move_player_bullets(&mut player_bullets);
                player_score += update_enemies(&mut asteroids, &bounds, &mut player_bullets, &tick);

                // make sure player is in bounds
                force_bounds_player(&mut player_position, &bounds);

                // delete the out of bounds bullets
                force_bounds_objects(&mut player_bullets, &bounds);

                if check_if_player_dead(&player_position, &asteroids)
                {
                    // game over!
                    scene = Scene::GameOver;
                }

                // draw game objects
                clear();
                draw_asteroids(&asteroids, &win_dimensions);
                draw_player(&player_position, &win_dimensions);
                draw_player_bullets(&player_bullets, &win_dimensions);

                draw_outline(win_x / 2 - bounds.x as i32 - 3, win_y / 2 - bounds.y as i32 - 1, 2 * bounds.x as i32 + 7, 2 * bounds.y as i32 + 6);
            }

            Scene::GameOver => 
            {
                draw_banner_center(&GAME_OVER_BANNER, &VEC_ZERO, &win_dimensions);
                thread::sleep(Duration::from_millis(100));
            }
        }


        // display score
        let mut message = format!("Score: {}", player_score);
        mv(win_dimensions.y / 2 + bounds.y + win_offset_y / 2 - 1, win_dimensions.x / 2 - bounds.x - win_offset_x / 2);
        addstr(&message);


        // draw debug overlay if needed
        if debug_overlay
            {
            message = format!("Player position: x: {}, y: {}", player_position.x, player_position.y);
            mv(0, 0);
            addstr(&message);

            message = format!("Bounds: x: {}, y: {}", bounds.x, bounds.y);
            mv(1, 0);
            addstr(&message);

            message = format!("Window size: x: {}, y: {}", win_x, win_y);
            mv(2, 0);
            addstr(&message);

            message = format!("Bullets: {}", player_bullets.len());
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
