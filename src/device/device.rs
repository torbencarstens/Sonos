use device::DeviceInfo;
use std::fmt;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Device {
    ip: SocketAddr,
    info: Box<DeviceInfo>
}

impl Device {
    pub fn new<T: Into<SocketAddr>>(ip_address: T) -> Self {
        let ip = ip_address.into();
        debug!("Creating new device with {:?}", ip);

        Device {
            ip,
            info: Box::new(DeviceInfo::new())
        }
    }
}

impl From<SocketAddr> for Device {
    fn from(addr: SocketAddr) -> Self {
        Device::new(addr)
    }
}

impl Into<SocketAddr> for Device {
    fn into(self) -> SocketAddr {
        self.ip
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<SonosDevice: {}>", self.ip)
    }
}
