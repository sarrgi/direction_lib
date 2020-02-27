#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DirectionOct {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/**
 * Function for converting primitive i32 values to a direction
 */
pub fn int_to_dir(val: i32) -> DirectionOct{
    return match val {
        0 => DirectionOct::North,
        1 => DirectionOct::NorthEast,
        2 => DirectionOct::East,
        3 => DirectionOct::SouthEast,
        4 => DirectionOct::South,
        5 => DirectionOct::SouthWest,
        6 => DirectionOct::West,
        7 => DirectionOct::NorthWest,
        _ => DirectionOct::North, //impossible case
    }
}

/**
 * return values in which points travel when facing a certain direction
 */
pub fn dir_to_point(val: DirectionOct) -> [f64; 2]{
    return match val {
        DirectionOct::North => [0.0, -1.0],
        DirectionOct::NorthEast => [1.0, -1.0],
        DirectionOct::East => [1.0, 0.0],
        DirectionOct::SouthEast => [1.0, 1.0],
        DirectionOct::South => [0.0, 1.0],
        DirectionOct::SouthWest => [-1.0, 1.0],
        DirectionOct::West => [-1.0, 0.0],
        DirectionOct::NorthWest => [-1.0, -1.0],
    }
}

/**
 * Takes a direction and returns the next clockwise direction from it.
 */
pub fn clockwise(val: DirectionOct) -> DirectionOct{
    return match val {
        DirectionOct::North => DirectionOct::NorthEast,
        DirectionOct::NorthEast => DirectionOct::East,
        DirectionOct::East => DirectionOct::SouthEast,
        DirectionOct::SouthEast => DirectionOct::South,
        DirectionOct::South => DirectionOct::SouthWest,
        DirectionOct::SouthWest => DirectionOct::West,
        DirectionOct::West => DirectionOct::NorthWest,
        DirectionOct::NorthWest => DirectionOct::North,
    }
}

/**
 * Takes a direction and returns the next anti-clockwise direction from it.
 */
pub fn anti_clockwise(val: DirectionOct) -> DirectionOct{
    return match val {
        DirectionOct::North => Direction::DirectionOct,
        DirectionOct::NorthEast => DirectionOct::North,
        DirectionOct::East => DirectionOct::NorthEast,
        DirectionOct::SouthEast => DirectionOct::East,
        DirectionOct::South => DirectionOct::SouthEast,
        DirectionOct::SouthWest => DirectionOct::South,
        DirectionOct::West => DirectionOct::SouthWest,
        DirectionOct::NorthWest => DirectionOct::West,
    }
}