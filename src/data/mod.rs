mod map;
mod train;

use freds::Inline;
pub use {map::Map, train::Train};
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
