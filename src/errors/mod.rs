mod base;

pub use self::base::SonosError;

#[derive(Debug)]
pub enum ErrorKind {
    DiscoveryError(String)
}
