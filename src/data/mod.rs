mod map;
mod stop;
mod train;
mod switch;

use freds::Inline;
pub use {map::Map, stop::Stop, switch::Switch, train::Train};

#[derive(Default, Debug)]
pub struct Line {
    point_a: Inline,
    point_b: Inline,
}
impl From<Line> for Vec<u8> {
    fn from(line: Line) -> Vec<u8> {
        let point_a: Vec<u8> = line.point_a.into();
        let point_b: Vec<u8> = line.point_b.into();
        [point_a, point_b].concat()
    }
}

#[derive(Default, Debug)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}
impl From<Position> for Vec<u8> {
    fn from(position: Position) -> Vec<u8> {
        let x: [u8; 8] = position.x.to_be_bytes();
        let y: [u8; 8] = position.y.to_be_bytes();
        [x, y].concat()
    }
}
