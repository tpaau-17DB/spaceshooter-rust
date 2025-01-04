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

lazy_static! 
{
    pub static ref ASTEROID_1: Vec<String> = 
    vec![
        "   -- ".to_string(),
        " /- '|".to_string(),
        "| .+/ ".to_string(),
        " \\_/ ".to_string(),
    ];
}

lazy_static! 
{
    pub static ref PLAYER: Vec<String> = 
    vec![
        "  .  ".to_string(),
        " /H\\".to_string(),
        "|H#H|".to_string(),
        "\\/ \\/".to_string(),
    ];
}

