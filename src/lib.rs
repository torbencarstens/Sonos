#[macro_use]
extern crate log;
extern crate sonos_discovery;

mod device;
mod errors;
mod zone;

use errors::SonosError;
use sonos_discovery::Discover;
use std::{collections, net};


pub fn discover(timeout: Option<u32>, device_count: Option<usize>) -> Result<collections::HashSet<net::IpAddr>, SonosError> {
    debug!("Start discovery");
    match Discover::new()?.start(timeout, device_count) {
        Ok(set) => {
            debug!("Successfully discovered {} devices: [{}]", set.len(), set.iter().map(|ip| format!("{}", ip)).collect::<Vec<String>>().join(", "));
            Ok(set)
        }
        Err(e) => {
            debug!("Failed to discover devices: {:#?}", e);
            Err(SonosError::from(e))
        }
    }
}
