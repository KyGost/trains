use {
    crate::Train,
    freds::{Data, Error, InlineData, ReferentialData},
};

pub struct Map {
    pub trains: Vec<Train>,
    //pub stops: Vec<Stop>,
    //pub switches: Vec<Switch>,
}

impl Data for Map {
    const KIND: [u8; 1] = [b'M'];
    const IS_INLINE: bool = false;
}
impl InlineData for Map {}
impl ReferentialData for Map {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        self.trains
            .into_iter()
            .map(|t| t.into_bytes())
            .collect::<Result<Vec<Vec<u8>>, _>>()
            .map(|a| a.concat())
    }
}
