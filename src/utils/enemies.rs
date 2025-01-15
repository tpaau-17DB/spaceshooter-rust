use rand::Rng;

use crate::utils::{
    collisions::*,
    movement::*,
    vectors::*,
    player::*,
};

pub struct BasicEnemy {
    pub damaged_last_tick: bool,
    pub health: i32,
    pub size: f32,
    pub damage: i32,
    pub position: Vector,
}


impl BasicEnemy {
    fn default() -> Self
    {
        BasicEnemy
        {
            damaged_last_tick: false,
            health: 4,
            size: 2.0,
            damage: 1,
            position: Vector{x: 0, y: 0},
        }
    }
}

pub struct EnemySpawner {
    pub asteroid_spawn_chance: f32,
}

static ENEMY_SPAWNER: EnemySpawner = EnemySpawner {
    // asteroid spawn chance in percent per tick !!!
    asteroid_spawn_chance: 5.0,
};

pub fn update_enemies(asteroids: &mut Vec<BasicEnemy>, bounds: &Vector,
    player: &mut Player, tick: &i32) -> i32 {

    let mut rng = rand::thread_rng();

    let rand_num: f32 = rng.gen_range(0.0..100.0); 
    if rand_num <= ENEMY_SPAWNER.asteroid_spawn_chance
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
        while j < player.bullets().len()
        {
            if intersect_circles(&asteroids[i].position, &asteroids[i].size, &player.bullets()[j], &1.0)
            {
                player.destroy_bullet(j);
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
