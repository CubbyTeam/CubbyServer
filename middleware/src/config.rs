//! Configuration of this connection
//!
//! # Examples
//!
//! ```
//! use cubby_server_middleware::config::{Config, AuthServer};
//!
//! // using only default values
//! let config = Config::builder().build().unwrap();
//!
//! // changing values
//! let config = Config::builder()
//!        .auth_config(AuthServer::builder().password("password").build().unwrap())
//!        .verbose(3)
//!        .build()
//!        .unwrap();
//! ```
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// configuration for auth server connection
#[derive(Builder, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[builder(derive(Debug, Deserialize, Eq, PartialEq, Serialize))]
pub struct AuthServer {
    /// host of auth server to connect to
    #[builder(default = "String::from(\"127.0.0.1\")", setter(into))]
    pub host: String,
    /// port of auth server to connect to
    ///
    /// todo: change this value to default port of auth server
    #[builder(default = "8080")]
    pub port: u16,
    /// username to login to auth server
    #[builder(default = "String::from(\"cubby-auth\")", setter(into))]
    pub username: String,
    /// password to login to auth server
    #[builder(default = "String::from(\"cubby-auth\")", setter(into))]
    pub password: String,
}

impl AuthServer {
    /// returns default builder of `AuthServer`
    pub fn builder() -> AuthServerBuilder {
        AuthServerBuilder::default()
    }
}

/// configuration for connection
#[derive(Builder, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[builder(derive(Debug, Deserialize, Eq, PartialEq, Serialize))]
pub struct Config {
    /// host to run this server
    #[builder(default = "(0, 0, 0, 0)")]
    pub host: (u8, u8, u8, u8),
    /// port to bind tcp connection
    #[builder(default = "20202")]
    pub tcp_port: u16,
    /// port to bind udp connection
    #[builder(default = "20302")]
    pub udp_port: u16,
    /// directory of protobuf files for connection
    #[builder(default = "PathBuf::from(\"./protobuf\")", setter(into))]
    pub protobuf_dir: PathBuf,
    /// key file of tls connection
    /// if this value is `None`, there is no tls connection
    #[builder(default = "None", setter(strip_option, into))]
    pub key_path: Option<PathBuf>,
    /// cert file of tls connection
    /// if this value is `None`, there is no tls connection
    #[builder(default = "None", setter(strip_option, into))]
    pub cert_path: Option<PathBuf>,
    /// auth server configuration
    #[builder(default = "AuthServer::builder().build().unwrap()")]
    pub auth_config: AuthServer,
    /// logging level of the server
    ///
    /// 0. don't print anything
    /// 1. print `error!`
    /// 2. print all above and print `warn!`
    /// 3. print all above and print `info!`
    /// 4. print all above and print `debug!`
    /// 5. print all above and print `trace!`
    #[builder(default = "3")]
    pub verbose: u8,
    /// **only for debug**
    ///
    /// If watch is true, server will watch protobuf files / configuration files
    /// and when they changes, server will restart.
    ///
    /// This value only shows up in compiling in debug mode.
    #[builder(default = "true")]
    #[cfg(debug_assertions)]
    pub watch: bool,
}

impl Config {
    /// returns default builder of `ConfigBuilder`
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}
