use errors::SonosError;
use super::futures;

#[derive(Debug)]
pub struct DeviceInfo {}

impl DeviceInfo {
    pub fn new() -> Self {
        debug!("Creating new DeviceInfo");
        DeviceInfo {}
    }
}

impl futures::Future for DeviceInfo {
    type Item = DeviceInfo;
    type Error = SonosError;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}
