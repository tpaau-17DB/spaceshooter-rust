extern crate ncurses;
use ncurses::*;

use std::thread;
use std::time::Duration;

mod utils;
use utils::drawing::*;
use utils::movement::*;

fn main() 
{
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    nodelay(stdscr(), true);
    setlocale(LcCategory::all, "en_US.UTF-8");
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let sleeptime_ms = 16;

    let gamewindow_x: i16 = 32;
    let gamewindow_y: i16 = 32;

    let win_offset_x: i16 = 7;
    let win_offset_y: i16 = 12;

    let bounds: Vec<i16> = [gamewindow_x / 2, gamewindow_y / 2].to_vec();

    let mut win_x = 0;
    let mut win_y = 0;
    getmaxyx(stdscr(), &mut win_y, &mut win_x);

    let window_to_small_message = "The terminal window is too small.";

    let mut player_position: Vec<i16> = [(win_x / 2) as i16, (win_y / 2) as i16].to_vec();

    loop 
    {
        let mut win_x = 0;
        let mut win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);

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
                    KEY_UP => player_position[1] -= 2,
                    KEY_DOWN => player_position[1] += 2,
                    KEY_LEFT => player_position[0] -= 3,
                    KEY_RIGHT => player_position[0] += 3,
                    _ => {},
                };
            }
        }

        // make sure player is in bounds
        force_bounds(&mut player_position, &bounds, &win_x, &win_y);

        clear();
        draw_player(&player_position);

        draw_border(win_x / 2 - bounds[0] as i32 - 3, win_y / 2 - bounds[1] as i32 - 1, 2 * bounds[0] as i32 + 7, 2 * bounds[1] as i32 + 6);

        let mut message = format!("Player position: x: {}, y: {}", player_position[0], player_position[1]);
        mv(win_y - 1, win_x - message.len() as i32);
        addstr(&message);

        message = format!("Bounds: x: {}, y: {}", bounds[0], bounds[1]);
        mv(win_y - 1, 0);
        addstr(&message);

        message = format!("Window size: x: {}, y: {}", win_x, win_y);
        mv(0, win_x - message.len() as i32);
        addstr(&message);

        refresh();

        thread::sleep(Duration::from_millis(sleeptime_ms));
    }

    endwin();
}
