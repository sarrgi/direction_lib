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
        0 => Direction::North,
        1 => Direction::NorthEast,
        2 => Direction::East,
        3 => Direction::SouthEast,
        4 => Direction::South,
        5 => Direction::SouthWest,
        6 => Direction::West,
        7 => Direction::NorthWest,
        _ => Direction::North, //impossible case
    }
}

/**
 * return values in which points travel when facing a certain direction
 */
pub fn dir_to_point(val: DirectionOct) -> [f64; 2]{
    return match val {
        Direction::North => [0.0, -1.0],
        Direction::NorthEast => [1.0, -1.0],
        Direction::East => [1.0, 0.0],
        Direction::SouthEast => [1.0, 1.0],
        Direction::South => [0.0, 1.0],
        Direction::SouthWest => [-1.0, 1.0],
        Direction::West => [-1.0, 0.0],
        Direction::NorthWest => [-1.0, -1.0],
    }
}

/**
 * Takes a direction and returns the next clockwise direction from it.
 */
pub fn clockwise(val: Direction) -> Direction{
    return match val {
        Direction::North => Direction::NorthEast,
        Direction::NorthEast => Direction::East,
        Direction::East => Direction::SouthEast,
        Direction::SouthEast => Direction::South,
        Direction::South => Direction::SouthWest,
        Direction::SouthWest => Direction::West,
        Direction::West => Direction::NorthWest,
        Direction::NorthWest => Direction::North,
    }
}

/**
 * Takes a direction and returns the next anti-clockwise direction from it.
 */
pub fn anti_clockwise(val: Direction) -> Direction{
    return match val {
        Direction::North => Direction::NorthWest,
        Direction::NorthEast => Direction::North,
        Direction::East => Direction::NorthEast,
        Direction::SouthEast => Direction::East,
        Direction::South => Direction::SouthEast,
        Direction::SouthWest => Direction::South,
        Direction::West => Direction::SouthWest,
        Direction::NorthWest => Direction::West,
    }
}