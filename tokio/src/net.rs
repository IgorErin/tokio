//! TCP/UDP/Unix bindings for `tokio`.
//!
//! This module contains the TCP/UDP/Unix networking types, similar to the standard
//! library, which can be used to implement networking protocols.
//!
//! # Organization
//!
//! * [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
//! * [`UdpSocket`] provides functionality for communication over UDP
//! * [`UnixListener`] and [`UnixStream`] provide functionality for communication over a
//! Unix Domain Stream Socket **(available on Unix only)**
//! * [`UnixDatagram`] and [`UnixDatagramFramed`] provide functionality for communication
//! over Unix Domain Datagram Socket **(available on Unix only)**

//!
//! [`TcpListener`]: struct.TcpListener.html
//! [`TcpStream`]: struct.TcpStream.html
//! [`UdpSocket`]: struct.UdpSocket.html
//! [`UnixListener`]: struct.UnixListener.html
//! [`UnixStream`]: struct.UnixStream.html
//! [`UnixDatagram`]: struct.UnixDatagram.html
//! [`UnixDatagramFramed`]: struct.UnixDatagramFramed.html

#[cfg(feature = "tcp")]
pub mod tcp {
    //! TCP bindings for `tokio`.
    //!
    //! Connecting to an address, via TCP, can be done using [`TcpStream`]'s
    //! [`connect`] method, which returns [`ConnectFuture`]. `ConnectFuture`
    //! implements a future which returns a `TcpStream`.
    //!
    //! To listen on an address [`TcpListener`] can be used. `TcpListener`'s
    //! [`incoming`][incoming_method] method can be used to accept new connections.
    //! It return the [`Incoming`] struct, which implements a stream which returns
    //! `TcpStream`s.
    //!
    //! [`TcpStream`]: struct.TcpStream.html
    //! [`connect`]: struct.TcpStream.html#method.connect
    //! [`ConnectFuture`]: struct.ConnectFuture.html
    //! [`TcpListener`]: struct.TcpListener.html
    //! [incoming_method]: struct.TcpListener.html#method.incoming
    //! [`Incoming`]: struct.Incoming.html
    pub use tokio_net::tcp::{split, TcpListener, TcpStream};
}
#[cfg(feature = "tcp")]
pub use self::tcp::{TcpListener, TcpStream};

#[cfg(feature = "udp")]
pub mod udp {
    //! UDP bindings for `tokio`.
    //!
    //! The main struct for UDP is the [`UdpSocket`], which represents a UDP socket.
    //!
    //! [`UdpSocket`]: struct.UdpSocket.html
    pub use tokio_net::udp::{split, UdpFramed, UdpSocket};
}
#[cfg(feature = "udp")]
pub use self::udp::{UdpFramed, UdpSocket};

#[cfg(feature = "signal")]
pub mod signal {
    //! Asynchronous signal handling for `tokio`. ctrl-C notifications are
    //! supported on both unix and windows systems. For finer grained signal
    //! handling support on unix systems, see `tokio_net::signal::unix::Signal`.
    pub use tokio_net::signal::ctrl_c;
}

#[cfg(feature = "process")]
pub mod process {
    //! An implementation of asynchronous process management for Tokio.
    pub use tokio_net::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
}

#[cfg(all(unix, feature = "uds"))]
pub mod unix {
    //! Unix domain socket bindings for `tokio` (only available on unix systems).

    pub use tokio_net::uds::{split, UCred, UnixDatagram, UnixListener, UnixStream};
}
#[cfg(all(unix, feature = "uds"))]
pub use self::unix::{UnixDatagram, UnixListener, UnixStream};
