use ncurses::*;
use crate::{utils::vectors::*, BasicEnemy};
use lazy_static::lazy_static;

lazy_static! 
{
    pub static ref GAME_OVER_BANNER: Vec<String> = 
    vec![
        "  ____    _    __  __ _____    _____     _______ ____  ".to_string(),
        " / ___|  / \\  |  \\/  | ____|  / _ \\ \\   / / ____|  _ \\ ".to_string(),
        "| |  _  / _ \\ | |\\/| |  _|   | | | \\ \\ / /|  _| | |_) |".to_string(),
        "| |_| |/ ___ \\| |  | | |___  | |_| |\\ V / | |___|  _ < ".to_string(),
        " \\____/_/   \\_\\_|  |_|_____|  \\___/  \\_/  |_____|_| \\_\\".to_string(),
    ];
}

pub fn draw_player(position: &Vector, window_dimensions: &Vector) 
{
    // transform local player position to on-screen position
    let pos_x = (position.x + window_dimensions.x / 2) as i32;
    let pos_y = (position.y + window_dimensions.y / 2) as i32;

    mv(pos_y, pos_x);
    addstr(".");
    mv(pos_y + 1, pos_x - 1);
    addstr("/0\\");
    mv(pos_y + 2, pos_x - 2);
    addstr("|H#H|");
    mv(pos_y + 3, pos_x - 2);
    addstr("\\/ \\/");
}

pub fn draw_player_bullets(bullets: &Vec<Vector>, win_dimensions: &Vector)
{
    for bullet in bullets
    {
        mv((bullet.y + win_dimensions.y / 2) as i32, (bullet.x + win_dimensions.x / 2) as i32);
        addstr("•");
    }
}

fn draw_asteroid(asteroid: &BasicEnemy, win_dimensions: &Vector)
{
    let x = asteroid.position.x as i32 + (win_dimensions.x / 2) as i32 - 3;
    let y = asteroid.position.y as i32 + (win_dimensions.y / 2) as i32 - 3;

    if asteroid.damaged_last_tick
    {
        attron(A_BOLD());
    }

    mv(y + 3, x);
    addstr("  --");
    mv(y + 4, x);
    addstr(" /- .|");
    mv(y + 5, x);
    addstr("| .+/");
    mv(y + 6, x);
    addstr(" \\_/");

    if asteroid.damaged_last_tick
    {
        attroff(A_BOLD());
    }
}

pub fn draw_asteroids(asteroids: &Vec<BasicEnemy>, win_dimensions: &Vector)
{
    for asteroid in asteroids
    {
        draw_asteroid(&asteroid, &win_dimensions);
    }
}

pub fn draw_outline(x: i32, y: i32, width: i32, height: i32) 
{
    mvaddstr(y, x, &"╭".to_string());  // Top-left corner
    mvaddstr(y, x + width - 1, &"╮".to_string()); // Top-right corner

    for i in x + 1..x + width - 1 
    {
        mvaddstr(y, i, &"─".to_string());
    }

    // Draw bottom border
    mvaddstr(y + height - 1, x, &"╰".to_string());  // Bottom-left corner
    mvaddstr(y + height - 1, x + width - 1, &"╯".to_string()); // Bottom-right corner

    for i in x + 1..x + width - 1 
    {
        mvaddstr(y + height - 1, i, &"─".to_string());
    }

    // Draw left and right borders
    for i in y + 1..y + height - 1 
    {
        mvaddstr(i, x, &"│".to_string());  // Left border
        mvaddstr(i, x + width - 1, &"│".to_string()); // Right border
    }
}

pub fn draw_banner(banner: &Vec<String>, win_dimensions: &Vector)
{
    let mut i: i32 = 0;
    while i < banner.len() as i32
    {
        mvaddstr(win_dimensions.y / 2 + i - banner.len() as i32 / 2, (win_dimensions.x - banner[i as usize].len() as i32) / 2, &banner[i as usize]);
        i += 1;
    }
}
