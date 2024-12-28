extern crate ncurses;
use ncurses::*;

use std::thread;
use std::time::Duration;

mod utils;
use utils::drawing::*;
use utils::movement::*;
use utils::vectors::*;
//use utils::enemies::*;

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

    // constants
    let debug_overlay = false;

    // 16 ms sleeptime means that game is running at 60 FPS.
    // 60 FPS is a bit of an overkill and I might change that later
    // this value is also assumed to be the delta time
    let sleeptime_ms = 16;

    // those are the dimensions of the in-game border
    let gamewindow_x: i16 = 32;
    let gamewindow_y: i16 = 32;

    let win_offset_x: i16 = 7;
    let win_offset_y: i16 = 12;

    // local Vector [0, 0] represents the center of the screen
    let mut player_position: Vector = Vector{x: 0, y: gamewindow_y / 2};

    let mut player_bullets: Vec<Vector> = Vec::new();
    let bullet_timeout_ms: i16 = 200;
    let mut bullet_cooldown = bullet_timeout_ms;

    //let mut asteroids: Vec<BasicEnemy> = Vec::new();

    //let bounds: Vector = [gamewindow_x / 2, gamewindow_y / 2].to_vec();
    let bounds: Vector = Vector{x: gamewindow_x / 2, y: gamewindow_y / 2};

    let mut win_x = 0;
    let mut win_y = 0;
    getmaxyx(stdscr(), &mut win_y, &mut win_x);

    let window_to_small_message = "The terminal window is too small.";

    // the main program loop, executed until 'q' is received
    loop 
    {
        let mut win_x = 0;
        let mut win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);

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


        // shoot!
        bullet_cooldown -= sleeptime_ms;
        if bullet_cooldown <= 0
        {
            bullet_cooldown = bullet_timeout_ms;

            player_bullets.push(Vector{x: player_position.x, y: player_position.y});
        }

        move_player_bullets(&mut player_bullets);
        //update_enemies(&mut asteroids);

        // make sure player is in bounds
        force_bounds_player(&mut player_position, &bounds);

        // delete the out of bounds bullets
        force_bounds_objects(&mut player_bullets, &bounds);

        // draw all the stuff
        clear();
        draw_player(&player_position);
        draw_player_bullets(&player_bullets);

        draw_border(win_x / 2 - bounds.x as i32 - 3, win_y / 2 - bounds.y as i32 - 1, 2 * bounds.x as i32 + 7, 2 * bounds.y as i32 + 6);

        // draw debug overlay if needed
        if debug_overlay
            {
            let mut message = format!("Player position: x: {}, y: {}", player_position.x, player_position.y);
            mv(win_y - 1, win_x - message.len() as i32);
            addstr(&message);

            message = format!("Bounds: x: {}, y: {}", bounds.x, bounds.y);
            mv(win_y - 1, 0);
            addstr(&message);

            message = format!("Window size: x: {}, y: {}", win_x, win_y);
            mv(0, win_x - message.len() as i32);
            addstr(&message);

            message = format!("Bullets: {}", player_bullets.len());
            mv(0, 0);
            addstr(&message);
        }

        refresh();

        thread::sleep(Duration::from_millis(sleeptime_ms as u64));
    }

    endwin();
}
