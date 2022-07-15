use {
    crate::Position,
    freds::{Data, Error, Inline, InlineData, ReferentialData},
};

#[derive(Default, Debug, Clone)]
pub struct Stop {
    pub position: Position,
    pub connection_a: Inline,
    pub connection_b: Inline,
}

impl Data for Stop {
    const KIND: [u8; 1] = [b'S'];
    const IS_INLINE: bool = false;
}
impl InlineData for Stop {}
impl ReferentialData for Stop {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        let connection_a: Vec<u8> = self.connection_a.into();
        let connection_b: Vec<u8> = self.connection_b.into();
        let position: Vec<u8> = self.position.into();
        Ok([connection_a, connection_b, position].concat())
    }
}
