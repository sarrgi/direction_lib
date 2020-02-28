#[cfg(test)]
mod tests {
    use crate::{Direction, clockwise, anti_clockwise, inverse};
    use crate::octagonol::{DirectionOct, oct_clockwise};

    #[test]
    fn clockwise_tests(){
        assert_eq!(clockwise(Direction::Up), Direction::Right);
        assert_eq!(clockwise(Direction::Down), Direction::Left);
        assert_eq!(clockwise(Direction::Left), Direction::Up);
        assert_eq!(clockwise(Direction::Right), Direction::Down);
    }

    #[test]
    fn anti_clockwise_tests(){
        assert_eq!(anti_clockwise(Direction::Up), Direction::Left);
        assert_eq!(anti_clockwise(Direction::Down), Direction::Right);
        assert_eq!(anti_clockwise(Direction::Left), Direction::Down);
        assert_eq!(anti_clockwise(Direction::Right), Direction::Up);
    }

    #[test]
    fn inverse_tests(){
        assert_eq!(inverse(Direction::Up), Direction::Down);
        assert_eq!(inverse(Direction::Down), Direction::Up);
        assert_eq!(inverse(Direction::Left), Direction::Right);
        assert_eq!(inverse(Direction::Right), Direction::Left);
    }

    //OCTAGONAL TESTS
    #[test]
    fn a(){
        let x = DirectionOct::North;
        assert_eq!(oct_clockwise(x), DirectionOct::NorthEast);

    }

}

//mod octagonol;

/**
 * 4 Way Direction enum. Look into expanding for use across multiple projects/making a crate.
 */
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up, Right, Down, Left
}

/**
 * Function for converting primitive i32 values to a direction
 */
pub fn int_to_dir(val: i32) -> Direction{
    return match val {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        3 => Direction::Left,
        _ => Direction::Up, //impossible case
    }
}

/**
 * Function for converting primitive f64 values to a direction
 */
pub fn float_to_dir(val: f64) -> Direction{
    return match val {
        0.0 => Direction::Up,
        1.0 => Direction::Right,
        2.0 => Direction::Down,
        3.0 => Direction::Left,
        _ => Direction::Up, //impossible case
    }
}

/**
 * Function for converting direction values to a primitive i32.
 */
pub fn dir_to_int(val: Direction) -> i32{
    return match val {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    }
}

/**
 * Function for converting direction values to a primitive f64.
 */
pub fn dir_to_float(val: Direction) -> f64{
    return match val {
        Direction::Up => 0.0,
        Direction::Right => 1.0,
        Direction::Down => 2.0,
        Direction::Left => 3.0,
    }
}

/**
 * return values in which points travel when facing a certain direction
 */
pub fn dir_to_point(val: Direction) -> [f64; 2]{
    return match val {
        Direction::Up => [0.0, -1.0],
        Direction::Down => [0.0, 1.0],
        Direction::Left => [-1.0, 0.0],
        Direction::Right => [1.0, 0.0],
    }
}

/**
 * Takes a direction and returns the next clockwise direction from it.
 */
pub fn clockwise(val: Direction) -> Direction{
    return match val {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

/**
 * Takes a direction and returns the next anti-clockwise direction from it.
 */
pub fn anti_clockwise(val: Direction) -> Direction{
    return match val {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    }
}

/**
 * Takes a direction and returns the opposite.
 */
pub fn inverse(val: Direction) -> Direction{
    return match val {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    }
}

////////////////////////////////////////////////////////////////////
// MOVED FROM OCT CLASS BC IT DOESN"T SEEM TO BE USBALE IN THERE? //
////////////////////////////////////////////////////////////////////
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
pub fn oct_int_to_dir(val: i32) -> DirectionOct{
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
pub fn oct_dir_to_point(val: DirectionOct) -> [f64; 2]{
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
pub fn oct_clockwise(val: DirectionOct) -> DirectionOct{
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
pub fn oct_anti_clockwise(val: DirectionOct) -> DirectionOct{
    return match val {
        DirectionOct::North => DirectionOct::NorthWest,
        DirectionOct::NorthEast => DirectionOct::North,
        DirectionOct::East => DirectionOct::NorthEast,
        DirectionOct::SouthEast => DirectionOct::East,
        DirectionOct::South => DirectionOct::SouthEast,
        DirectionOct::SouthWest => DirectionOct::South,
        DirectionOct::West => DirectionOct::SouthWest,
        DirectionOct::NorthWest => DirectionOct::West,
    }
}

pub fn oct_is_diagonal(val: DirectionOct) -> bool {
    return match val{
        DirectionOct::North => false,
        DirectionOct::NorthEast => true,
        DirectionOct::East => false,
        DirectionOct::SouthEast => true,
        DirectionOct::South => false,
        DirectionOct::SouthWest => true,
        DirectionOct::West => false,
        DirectionOct::NorthWest => true,
    }
}
