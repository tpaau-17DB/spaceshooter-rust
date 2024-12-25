// force an object to stay in bounds
pub fn force_bounds(position: &mut Vec<i16>, bounds: &Vec<i16>, win_x: &i32, win_y: &i32)
{
    if position.len() == 2 && bounds.len() == 2
    {
        if position[0] > bounds[0] + *win_x as i16 / 2
        {
            position[0] = *win_x as i16 / 2 + bounds[0];
        }
        else if position[0] < *win_x as i16 / 2 - bounds[0]
        {
            position[0] = *win_x as i16 / 2 - bounds[0]
        }

        if position[1] > *win_y as i16 / 2 + bounds[1]
        {
            position[1] = *win_y as i16 / 2 + bounds[1];
        }
        else if position[1] < *win_y as i16 / 2 - bounds[1]
        {
            position[1] = *win_y as i16 / 2 - bounds[1];
        }
    }
    else 
    {
        println!("error: either position or bounds vectors have invalid length. position.len(): {}, bounds.len(): {}", position.len(), bounds.len());
    }
}
