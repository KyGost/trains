use {
    crate::Line,
    freds::{Data, Error, InlineData, ReferentialData},
};

#[derive(Default, Debug, Clone)]
pub struct Train {
    pub orientation: bool,
    pub speed: u64,
    pub position: u64,
    pub line: Line,
}

impl Data for Train {
    const KIND: [u8; 1] = [b'T'];
    const IS_INLINE: bool = false;
}
impl InlineData for Train {}
impl ReferentialData for Train {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        let orientation: [u8; 1] = [self.orientation.into()];
        let speed: [u8; 8] = self.speed.to_be_bytes();
        let position: [u8; 8] = self.position.to_be_bytes();
        let line: Vec<u8> = self.line.into();

        Ok([
            orientation.to_vec(),
            speed.to_vec(),
            position.to_vec(),
            line,
        ]
        .concat())
    }
}
