#[cfg(not(target_env = "fortanixvme"))]
mod datagram;
#[cfg(not(target_env = "fortanixvme"))]
pub use self::datagram::UnixDatagram;

#[cfg(not(target_env = "fortanixvme"))]
mod listener;
#[cfg(not(target_env = "fortanixvme"))]
pub use self::listener::UnixListener;

#[cfg(not(target_env = "fortanixvme"))]
mod stream;
#[cfg(not(target_env = "fortanixvme"))]
pub use self::stream::UnixStream;
pub use crate::sys::SocketAddr;
