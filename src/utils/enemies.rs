use crate::{is_in_bounds, Vector};
use rand::Rng;

pub struct BasicEnemy
{
    pub health: u16,
    pub size: f32,
    pub position: Vector,
}

pub fn update_enemies(asteroids: &mut Vec<BasicEnemy>, bounds: &Vector, tick: &i32)
{
    // asteroid enemy spawn chance per tick in percent
    let asteroid_spawn_chance : f32 = 0.8;

    let mut rng = rand::thread_rng();

    let rand_num: f32 = rng.gen_range(0.0..100.0); 
    if rand_num <= asteroid_spawn_chance
    {
        asteroids.push(BasicEnemy{health: 3, size: 4.0, position: Vector{x: rng.gen_range(-16..16), y: -16}});
    }

    // move objects and delete those out of bounds
    let mut i = 0;
    let move_asteroids = tick % 3 == 0;
    while i < asteroids.len()
    {
        if move_asteroids
        {
            asteroids[i].position.y += 1;
        }

        if !is_in_bounds(&asteroids[i].position, &bounds)
        {
            asteroids.remove(i);
        }
        i += 1;
    }
}
