use crate::utils::{
    vectors::*,
    player::*,
};

// force an object to stay in bounds
pub fn force_bounds_player(player: &mut Player, bounds: &Vector)
{
    if player.position.x > bounds.x
    {
        player.position.x = bounds.x;
    }
    else if player.position.x < -bounds.x
    {
        player.position.x = -bounds.x
    }
    if player.position.y > bounds.y
    {
        player.position.y = bounds.y;
    }
    else if player.position.y < -bounds.y
    {
        player.position.y = -bounds.y;
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
