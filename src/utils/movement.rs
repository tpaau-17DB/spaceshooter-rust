use crate::utils::vectors::*;

pub fn move_player_bullets(bullets: &mut Vec<Vector>)
{
    for bullet in bullets
    {
        bullet.y -= 2;
    }
}

// force an object to stay in bounds
pub fn force_bounds_player(position: &mut Vector, bounds: &Vector)
{
    if position.x > bounds.x
    {
        position.x = bounds.x;
    }
    else if position.x < -bounds.x
    {
        position.x = -bounds.x
    }

    if position.y > bounds.y
    {
        position.y = bounds.y;
    }
    else if position.y < -bounds.y
    {
        position.y = -bounds.y;
    }
}

pub fn is_in_bounds(object: &Vector, bounds: &Vector) -> bool
{
    if object.x > bounds.x 
    {
        return false;
    }
    else if object.x < -bounds.x
    {
        return false;
    }
    if object.y > bounds.y
    {
        return false;
    }
    else if object.y < -bounds.y
    {
        return false;
    }
    return true;
}

pub fn force_bounds_objects(objects: &mut Vec<Vector>, bounds: &Vector)
{
    objects.retain(|object| is_in_bounds(object, bounds));
}
