extern crate ncurses;

use std::thread;
use std::time::Duration;

use ncurses::*;

fn draw_player(position: &Vec<i16>) 
{
    if position.len() == 2
    {
        let mut win_x = 0;
        let mut win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);

        mv(position[1] as i32, position[0] as i32);
        addstr(".");
        mv(position[1] as i32 + 1, position[0] as i32 - 1);
        addstr("/0\\");
        mv(position[1] as i32 + 2, position[0] as i32 - 2);
        addstr("|H#H|");
        mv(position[1] as i32 + 3, position[0] as i32 - 2);
        addstr("\\/ \\/");
    }
}

fn draw_border(x: i32, y: i32, width: i32, height: i32) 
{
    mvaddstr(y, x, &"+".to_string());  // Top-left corner
    mvaddstr(y, x + width - 1, &"+".to_string()); // Top-right corner

    for i in x + 1..x + width - 1 
    {
        mvaddstr(y, i, &"-".to_string());
    }

    // Draw bottom border
    mvaddstr(y + height - 1, x, &"+".to_string());  // Bottom-left corner
    mvaddstr(y + height - 1, x + width - 1, &"+".to_string()); // Bottom-right corner

    for i in x + 1..x + width - 1 
    {
        mvaddstr(y + height - 1, i, &"-".to_string());
    }

    // Draw left and right borders
    for i in y + 1..y + height - 1 
    {
        mvaddstr(i, x, &"|".to_string());  // Left border
        mvaddstr(i, x + width - 1, &"|".to_string()); // Right border
    }
}

// force an object to stay in bounds
fn force_bounds(position: &mut Vec<i16>, bounds: &Vec<i16>, win_x: &i32, win_y: &i32)
{
    // safety checks :)
    if position.len() == 2 && bounds.len() == 2
    {
        if position[0] > bounds[0] + *win_x as i16 / 2
        {
            position[0] = *win_x as i16 / 2 + bounds[0];
        }
        else if position[0] < *win_x as i16 / 2 - bounds[0]
        {
            position[0] = *win_x as i16 / 2 - bounds[0]
        }

        if position[1] > *win_y as i16 / 2 + bounds[1]
        {
            position[1] = *win_y as i16 / 2 + bounds[1];
        }
        else if position[1] < *win_y as i16 / 2 - bounds[1]
        {
            position[1] = *win_y as i16 / 2 - bounds[1];
        }
    }
    else 
    {
        println!("error: either position or bounds vectors have invalid length. position.len(): {}, bounds.len(): {}", position.len(), bounds.len());
    }
}

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
