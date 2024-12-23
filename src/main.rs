extern crate ncurses;

use std::thread;
use std::time::Duration;

use ncurses::*;

fn draw_player(position: &Vec<i8>) 
{
    if position.len() == 2
    {
        let mut win_x = 0;
        let mut win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);

        mv(position[1] as i32, position[0] as i32);
        addstr(".");
        mv(position[1] as i32 + 1, position[0] as i32 - 1);
        addstr("/H\\");
        mv(position[1] as i32 + 2, position[0] as i32 - 2);
        addstr("|H#H|");
        mv(position[1] as i32 + 3, position[0] as i32 - 2);
        addstr("\\/ \\/");
    }
}

fn force_bounds(position: &mut Vec<i8>)
{
    if position.len() == 2
    {
        let mut win_x = 0;
        let mut win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);
    }
    else 
    {
        println!("error: expected a vector with 2 elements, got a vector with {} elements!", position.len());
    }
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

    let mut player_position: Vec<i8> = [(win_x / 2) as i8, (win_y / 2) as i8].to_vec();

    loop 
    {
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
                    KEY_UP => player_position[1] -= 1,
                    KEY_DOWN => player_position[1] += 1,
                    KEY_LEFT => player_position[0] -= 1,
                    KEY_RIGHT => player_position[0] += 1,
                    _ => {},
                };
            }
        }

        // make sure player is in bounds
        force_bounds(&mut player_position);


        clear();
        draw_player(&player_position);

        refresh();

        thread::sleep(Duration::from_millis(sleeptime_ms));
    }

    endwin();
}
