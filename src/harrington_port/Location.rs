
// PORT COMPLETE

enum Location {
    N,
    W,
    E,
    S,
    NW,
    NE,
    SW,
    SE,
    C,
}

pub fn getOppositeDirection(location: Location) -> Location {
    return match location {
        Location::N => Location::S,
        Location::W => Location::E,
        Location::E => Location::W,
        Location::S => Location::N,
        Location::NW => Location::SE,
        Location::NE => Location::SW,
        Location::SW => Location::NE,
        Location::SE => Location::NW,
        Location::C => Location::C,
    }
}
