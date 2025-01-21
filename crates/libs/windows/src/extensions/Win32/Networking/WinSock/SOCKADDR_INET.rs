use crate::Win32::Networking::WinSock::SOCKADDR_INET;

impl From<core::net::SocketAddrV4> for SOCKADDR_INET {
    fn from(addr: core::net::SocketAddrV4) -> Self {
        SOCKADDR_INET { Ipv4: addr.into() }
    }
}
impl From<core::net::SocketAddrV6> for SOCKADDR_INET {
    fn from(addr: core::net::SocketAddrV6) -> Self {
        SOCKADDR_INET { Ipv6: addr.into() }
    }
}
impl From<core::net::SocketAddr> for SOCKADDR_INET {
    fn from(addr: core::net::SocketAddr) -> Self {
        match addr {
            core::net::SocketAddr::V4(socket_addr_v4) => socket_addr_v4.into(),
            core::net::SocketAddr::V6(socket_addr_v6) => socket_addr_v6.into(),
        }
    }
}
