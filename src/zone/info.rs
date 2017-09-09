use errors::SonosError;
use super::futures;

#[derive(Debug)]
pub struct ZoneInfo {}

impl ZoneInfo {
    pub fn empty() -> Self {
        debug!("Creating new ZoneInfo");
        ZoneInfo {}
    }
}

impl futures::Future for ZoneInfo {
    type Item = ZoneInfo;
    type Error = SonosError;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}
