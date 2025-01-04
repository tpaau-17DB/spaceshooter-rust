use ncurses::*;

use crate::utils::
{
    vectors::*,
    enemies::*,
    banners::*,
};

pub fn draw_player(position: &Vector, window_dimensions: &Vector) 
{
    // transform local player position to on-screen position
    let pos = Vector{x: (position.x + window_dimensions.x / 2) - 2 as i32, y: (position.y + window_dimensions.y / 2) + 2 as i32};

    draw_banner(&PLAYER, &pos);
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
    let y = asteroid.position.y as i32 + (win_dimensions.y / 2) as i32 + 2;

    if asteroid.damaged_last_tick
    {
        attron(A_BOLD());
    }

    draw_banner(&ASTEROID_1, &Vector{y, x});

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

pub fn draw_banner(banner: &Vec<String>, pos: &Vector)
{
    let mut i: i32 = 0;
    while i < banner.len() as i32
    {
        mvaddstr(pos.y - banner.len() as i32 / 2 + i, pos.x, &banner[i as usize]);
        i += 1;
    }
}

pub fn draw_banner_center(banner: &Vec<String>, offset: &Vector, win_dimensions: &Vector)
{
    let mut i: i32 = 0;
    while i < banner.len() as i32
    {
        mvaddstr(win_dimensions.y / 2 + i - banner.len() as i32 / 2 + offset.y, (win_dimensions.x - banner[i as usize].len() as i32) / 2 + offset.y, &banner[i as usize]);
        i += 1;
    }
}
