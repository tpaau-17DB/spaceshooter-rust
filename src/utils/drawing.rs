use ncurses::*;

pub fn draw_player(position: &Vec<i16>) 
{
    if position.len() == 2
    {
        // transform local player position to on-screen gosition
        let mut win_x = 0;
        let mut win_y = 0;
        getmaxyx(stdscr(), &mut win_y, &mut win_x);
        let pos_x = position[0] as i32 + win_x / 2;
        let pos_y = position[1] as i32 + win_y / 2;

        mv(pos_y, pos_x);
        addstr(".");
        mv(pos_y + 1, pos_x - 1);
        addstr("/0\\");
        mv(pos_y + 2, pos_x - 2);
        addstr("|H#H|");
        mv(pos_y + 3, pos_x - 2);
        addstr("\\/ \\/");
    }
}

pub fn draw_border(x: i32, y: i32, width: i32, height: i32) 
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
