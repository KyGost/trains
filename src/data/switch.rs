use {
    crate::Position,
    freds::{Data, Error, Inline, InlineData, ReferentialData},
};

#[derive(Default, Debug, Clone)]
pub struct Switch {
    pub position: Position,
    pub connection_a: Inline,
    pub connection_b: Inline,
    pub connection_z: Inline,
}

impl Data for Switch {
    const KIND: [u8; 1] = [b'S'];
    const IS_INLINE: bool = false;
}
impl InlineData for Switch {}
impl ReferentialData for Switch {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        let connection_a: Vec<u8> = self.connection_a.into();
        let connection_b: Vec<u8> = self.connection_b.into();
        let connection_z: Vec<u8> = self.connection_z.into();
        let position: Vec<u8> = self.position.into();
        Ok([connection_a, connection_b, connection_z, position].concat())
    }
}
