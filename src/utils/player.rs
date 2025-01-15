use crate::utils::{
    vectors::*,
    enemies::*,
    collisions::*,
    movement::*
};

pub struct Player<'a> {
    pub position: Vector,
    bounds: &'a Vector,
    pub score: i32,
    pub size: f32,
    health: i32,
    dead: bool,

    bullets: Vec<Vector>,
    shoot_freq: i32,
    shoot_cooldown: i32,
    bullet_speed: i32,
}

impl<'a> Player<'a> {
    pub fn default() -> Self {
        return Player {
            position: VEC_ZERO.clone(),
            size: 2.0,
            score: 0,
            health: 1,
            bounds: &VEC_ZERO,
            dead: false,
        
            bullets: Vec::new(),
            shoot_freq: 7,
            shoot_cooldown: 0,
            bullet_speed: 2,
        };
    }

    pub fn get_damaged(&mut self, damage: i32) {
        if self.health <= 0 {
            return;
        }
        self.health -= damage;
        if self.health <= 0 {
            self.dead = true;
        }
    }

    pub fn tick(&mut self, enemies: &Vec<BasicEnemy>) {
        for enemy in enemies {
            if intersect_circles(&self.position, &self.size, &enemy.position, &enemy.size)
            {
                self.get_damaged(enemy.damage);
            }
        }

        self.shoot_cooldown -= 1;

        if self.shoot_cooldown <= 0
        {
            self.shoot_cooldown = self.shoot_freq;
            self.bullets.push(Vector{x: self.position.x, y: self.position.y});
        }

        let mut i = 0;
        while i < self.bullets.len() {
            self.bullets[i].y -= self.bullet_speed;
            if !is_in_bounds(&self.bullets[i], &self.bounds) {
                self.destroy_bullet(i);
            }
            i += 1;
        }
    }

    pub fn destroy_bullet(&mut self, index: usize) {
        self.bullets.remove(index);
    }

    pub fn set_bounds(&mut self, bounds: &'a Vector) {
        self.bounds = bounds;
    }

    // GETTERS
    pub fn dead(&self) -> bool { self.dead }
    pub fn bullets(&self) -> &Vec<Vector> { &self.bullets }
}
