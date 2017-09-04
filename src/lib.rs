extern crate sonos_discovery;

mod errors;

use errors::SonosError;
use sonos_discovery::Discover;
use std::{collections, net};


pub fn discover(timeout: Option<u32>, device_count: Option<usize>) -> Result<collections::HashSet<net::IpAddr>, SonosError> {
    match Discover::new()?.start(timeout, device_count) {
        Ok(set) => Ok(set),
        Err(e) => Err(SonosError::from(e))
    }
}
