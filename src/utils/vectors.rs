pub struct Vector 
{
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn clone(&self) -> Self {
        return Self {
            x: self.x, y: self.y,
        }
    }
}

pub static VEC_ZERO: Vector = Vector{x: 0, y: 0};
