//! Connects server and client with simple API.
//!
//! It uses TCP & UDP to connect and transfer data.
//! Also available for secure connection using TLS.
//!
//! # Features
//!
//! - fast UDP connection
//! - secure TCP connection using TLS
//! - transfers data using protobuf
//! - pinging for heartbeat
//! - reconnection when internet is temporary disabled (in client)
//! - functional API that can be called in server & client
//! - connection to credential server for authentication
//! - version matching for compatability
//! - beautiful logging support
//!

#[macro_use]
extern crate derive_builder;

pub mod config;

pub const VERSION: (u16, u16, u16) = (0, 1, 0);
