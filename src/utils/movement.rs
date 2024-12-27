use crate::utils::vectors::*;

pub fn move_player_bullets(bullets: &mut Vec<vector>)
{
    for bullet in bullets
    {
        bullet.y -= 1;
    }
}

// force an object to stay in bounds
pub fn force_bounds_player(position: &mut Vec<i16>, bounds: &Vec<i16>)
{
    if position.len() == 2 && bounds.len() == 2
    {
        if position[0] > bounds[0]
        {
            position[0] = bounds[0];
        }
        else if position[0] < -bounds[0]
        {
            position[0] = -bounds[0]
        }

        if position[1] > bounds[1]
        {
            position[1] = bounds[1];
        }
        else if position[1] < -bounds[1]
        {
            position[1] = -bounds[1];
        }
    }
    else 
    {
        println!("error: either position or bounds vectors have invalid length. position.len(): {}, bounds.len(): {}", position.len(), bounds.len());
    }
}

pub fn is_in_bounds(object: &vector, bounds: &Vec<i16>) -> bool
{
    if object.x > bounds[0] 
    {
        return false;
    }
    else if object.x < -bounds[0]
    {
        return false;
    }
    if object.y > bounds[1]
    {
        return false;
    }
    else if object.y < -bounds[1]
    {
        return false;
    }
    return true;
}

pub fn force_bounds_objects(objects: &mut Vec<vector>, bounds: &Vec<i16>)
{
    objects.retain(|object| is_in_bounds(object, bounds));
}
