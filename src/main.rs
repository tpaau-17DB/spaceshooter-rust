extern crate ncurses;

use std::thread;
use std::time::Duration;

use ncurses::*;

fn draw_player(player_pos: &Vec<i8>) 
{
    let mut win_x = 0;
    let mut win_y = 0;
    getmaxyx(stdscr(), &mut win_y, &mut win_x);
    
    mv(player_pos[1] as i32, player_pos[0] as i32);
    addstr("*");
}

fn main() 
{
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let sleeptime_ms = 16;

    let min_x = 16;
    let min_y = 32;

    let mut win_x = 0;
    let mut win_y = 0;
    getmaxyx(stdscr(), &mut win_y, &mut win_x);

    let window_to_small_message = "The terminal window is too small.";

    let mut player_pos: Vec<i8> = [(win_x / 2) as i8, (win_y / 2) as i8].to_vec();

    loop 
    {
        //clear();

        let mut win_x = 0;
        let mut win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);

        if win_y < min_y || win_x < min_x 
        {
            let msg = format!("Required size: x: {min_x}, y: {min_y}");

            mv(win_y / 2, (win_x - window_to_small_message.len() as i32) / 2);
            addstr(&window_to_small_message);
            mv(win_y / 2 + 1, (win_x - msg.len() as i32) / 2);
            addstr(&msg);
        }
        else
        {
            let input = getch();

            match input 
            {
                val if val == ('q' as i32) => break,
                _ => {
                    match input as i32 {
                        KEY_UP => player_pos[1] -= 1,
                        KEY_DOWN => player_pos[1] += 1,
                        KEY_LEFT => player_pos[0] -= 1,
                        KEY_RIGHT => player_pos[0] += 1,
                        _ => {},
                    };
                }
            }

        }

        draw_player(&player_pos);

        refresh();

        thread::sleep(Duration::from_millis(sleeptime_ms));
    }

    endwin();

    println!("Player pos: x: {}, y: {}", player_pos[0], player_pos[1]);
}
