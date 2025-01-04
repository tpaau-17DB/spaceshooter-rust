use rand::Rng;

use crate::utils::
{
    vectors::*,
    movement::*,
    collisions::*,
};

pub struct BasicEnemy
{
    pub damaged_last_tick: bool,
    pub health: i32,
    pub size: f32,
    pub position: Vector,
}

impl Default for BasicEnemy
{
    fn default() -> Self
    {
        BasicEnemy
        {
            damaged_last_tick: false,
            health: 4,
            size: 2.0,
            position: Vector{x: 0, y: 0},
        }
    }
}

pub fn update_enemies(asteroids: &mut Vec<BasicEnemy>, bounds: &Vector, player_bullets: &mut Vec<Vector>, tick: &i32) -> i32
{
    // asteroid enemy spawn chance per tick in percent
    let asteroid_spawn_chance : f32 = 5.0;

    let mut rng = rand::thread_rng();

    let rand_num: f32 = rng.gen_range(0.0..100.0); 
    if rand_num <= asteroid_spawn_chance
    {
        let mut asteroid = BasicEnemy::default();
        asteroid.position = Vector{x: rng.gen_range(-16..16), y: -16};
        asteroids.push(asteroid);
    }

    let mut score = 0;

    // move objects and delete those out of bounds
    let mut i = 0;
    let move_asteroids = tick % 3 == 0;
    while i < asteroids.len()
    {
        asteroids[i].damaged_last_tick = false;

        if move_asteroids
        {
            asteroids[i].position.y += 1;
        }

        if !is_in_bounds(&asteroids[i].position, &bounds)
        {
            asteroids.remove(i);
            continue;
        }

        let mut j = 0;
        while j < player_bullets.len()
        {
            if intersect_circles(&asteroids[i].position, &asteroids[i].size, &player_bullets[j], &1.0)
            {
                player_bullets.remove(j);
                asteroids[i].position.y -= 1;
                asteroids[i].damaged_last_tick = true;
                asteroids[i].health -= 1;
                score += 5;

                if asteroids[i].health <= 0
                {
                    asteroids.remove(i);
                    score += 25;
                    break;
                }
            }
            else
            {
                j += 1;
            }
        }
        i += 1;
    }

    return score;
}

pub fn check_if_player_dead(player_position: &Vector, asteroids: &Vec<BasicEnemy>) -> bool
{
    for asteroid in asteroids
    {
        if intersect_circles(&player_position, &2.0, &asteroid.position, &asteroid.size)
        {
            return true;
        }
    }

    return false;
}
