//! This module contains the packets that are expected to never change
//!
//! This includes the legacy server list ping, handshake packet and status state and packets
//!
//! All of these packets can be used with clients/servers of any protocol version
//!

pub mod c2s;
pub mod s2c;
