extern crate sonos_discovery;

mod errors;

use errors::SonosError;
use sonos_discovery::Discover;


pub fn discover(timeout: Option<u32>, device_count: Option<usize>) -> Result<std::collections::HashSet<std::net::IpAddr>, SonosError> {
    match Discover::new()?.start(timeout, device_count) {
        Ok(set) => Ok(set),
        Err(e) => Err(SonosError::from(e))
    }
}
