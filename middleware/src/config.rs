//! Configuration of this connection
//!
//! # Examples
//!
//! ```
//! use cubby_server_middleware::config::{Config, AuthServer, make_tls_config};
//!
//! // using only default values
//! let config = Config::builder().build().unwrap();
//!
//! // changing values
//! let config = Config::builder()
//!        .auth_config(AuthServer::builder().password("password").build().unwrap())
//!        .build()
//!        .unwrap();
//!
//! let config = Config::builder()
//!        .tls_config(make_tls_config("key.pem", "cert.pem"))
//!        .build()
//!        .unwrap();
//! ```
use custom_debug_derive::Debug;
use std::path::{Path, PathBuf};

/// configuration for auth server connection
#[derive(Builder, Debug, Clone)]
#[builder(derive(Debug, PartialEq, Eq))]
pub struct AuthServer {
    // host of auth server to connect to
    #[builder(default = "String::from(\"127.0.0.1\")", setter(into))]
    pub host: String,
    // port of auth server to connect to
    #[builder(default = "8080")]
    pub port: u16,
    // username to login to auth server
    #[builder(default = "String::from(\"cubby-auth\")", setter(into))]
    pub username: String,
    // password to login to auth server
    #[builder(default = "String::from(\"cubby-auth\")", setter(into))]
    pub password: String,
}

impl AuthServer {
    /// returns builder of `AuthServer`
    pub fn builder() -> AuthServerBuilder {
        AuthServerBuilder::default()
    }
}

/// configuration for connection
#[derive(Builder, Clone, Debug)]
pub struct Config {
    // host to run this server
    #[builder(default = "(0, 0, 0, 0)")]
    pub host: (u8, u8, u8, u8),
    // port to bind tcp connection
    #[builder(default = "20202")]
    pub tcp_port: u16,
    // port to bind udp connection
    #[builder(default = "20302")]
    pub udp_port: u16,
    // directory of protobuf files for connection
    #[builder(default = "PathBuf::from(\"./protobuf\")", setter(into))]
    pub protobuf_dir: PathBuf,
    // tls configuration
    // if this value is `None`, there is no secure connection
    #[builder(
        setter(strip_option),
        default = "Some(make_tls_config(\"key.pem\", \"cert.pem\"))"
    )]
    #[debug(skip)]
    pub tls_config: Option<rustls::ServerConfig>,
    // auth server configuration
    #[builder(default = "AuthServer::builder().build().unwrap()")]
    pub auth_config: AuthServer,
    // logging level of the server
    #[builder(default = "2")]
    pub verbose: u8,
    // **only for debug**
    // if watch is true, server will watch protobuf files / configuration files
    // and when they changes, server will restart
    #[builder(default = "true")]
    #[cfg(debug_assertions)]
    pub watch: bool,
}

impl Config {
    /// returns builder of `ConfigBuilder`
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

/// Generates tls configuration from private & public keys
///
/// - `key_path`: path to key files (e.g. `key.pem`)
/// - `cert_path`: path to cert files (e.g. `cert.pem`)
///
/// We will use TLS 1.3 for connection, ECDSA for handshake, AES256 for encryption
pub fn make_tls_config<P1, P2>(key_path: P1, cert_path: P2) -> rustls::ServerConfig
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    todo!(
        "{} {}",
        key_path.as_ref().as_os_str().to_str().unwrap(),
        cert_path.as_ref().as_os_str().to_str().unwrap()
    )
}
