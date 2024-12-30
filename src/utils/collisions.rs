use crate::Vector;

pub fn intersect_circles(pos_a: &Vector, rad_a: &f32, pos_b: &Vector, rad_b: &f32) -> bool
{
    let d2x = pos_b.x - pos_a.x;
    let d2y = pos_b.y - pos_a.y;
    let distance_squared = d2x * d2x + d2y * d2y;

    let radii_sum_squared = (rad_a + rad_b) * (rad_a + rad_b);

    return distance_squared as f32 <= radii_sum_squared
}
