//! RethinkDB Driver
//!
//! ```rust
//! extern crate reql;
//! extern crate thinker;
//!
//! use reql::*;
//! use thinker::r;
//!
//! # fn main() {
//! r.connect(ConnectOpts::default()).unwrap();
//! # }
//! ```

#![allow(non_upper_case_globals)]

extern crate ql2;
extern crate reql;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
extern crate byteorder;
extern crate bufstream;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate slog;
extern crate slog_term;

pub mod conn;
pub mod types;

use reql::*;
use conn::ConnectionManager;
use std::sync::RwLock;
use slog::DrainExt;
use r2d2::{Pool, Config as PoolConfig};

pub struct Session{
    pub config: RwLock<SessionConfig>,
}

#[derive(Debug)]
pub struct SessionConfig{
    pub pool: Option<Pool<ConnectionManager>>,
    pub logger: slog::Logger,
}

lazy_static! {
    pub static ref session: Session = Session{
        config: RwLock::new(SessionConfig::new()),
    };
}

pub struct Reql;

pub const r: Reql = Reql;

impl SessionConfig {
    pub fn new() -> SessionConfig {
        SessionConfig{
            pool: None,
            logger: slog::Logger::root(
                slog_term::streamer().full().build().fuse(),
                o!("version" => env!("CARGO_PKG_VERSION"))
                ),
        }
    }
}

impl R for Reql {
    /// Creates a connection pool
    fn connect<T: IntoConnectOpts>(&self, opts: T) -> Result<()> {
        // If pool is already set we do nothing
        {
            let cfg = try!(session.config.read().map_err(|err| {
                let msg = format!("failed to acquire read lock to the session config: {}", err);
                ConnectionError::PoolRead(msg)
            }));
            if cfg.pool.is_some() {
                return Ok(());
            }
            info!(cfg.logger, "Trying to create a connection pool...");
        }
        // Otherwise we set it
        let manager = ConnectionManager::new(opts);
        let pool = try!(Pool::new(PoolConfig::default(), manager));
        let mut cfg = try!(session.config.write().map_err(|err| {
            let msg = format!("failed to acquire write lock to the session config: {}", err);
            ConnectionError::PoolWrite(msg)
        }));
        cfg.pool = Some(pool);
        Ok(())
    }
}

fn table(name: &str) -> Result<()> {
    Ok(())
}

pub trait Run {
    fn run(self) -> Self;
}

impl Run for Result<String> {
    fn run(self) -> Result<String> {
        let mut pool_is_empty = false;
        {
            let config = try!(session.config.read().map_err(|err| {
                let msg = format!("failed to acquire read lock to the session config: {}", err);
                ConnectionError::PoolRead(msg)
            }));
            if config.pool.is_none() {
                pool_is_empty = true;
            }
        }
        // If pool is empty we will use default options
        if pool_is_empty {
            try!(r.connect(ConnectOpts::default()));
        }
        let ref pool = try!(session.config.read().map_err(|err| {
                let msg = format!("failed to acquire read lock to the session config: {}", err);
                ConnectionError::PoolRead(msg)
            })).pool;
        match *pool {
            Some(ref p) => { 
                let _conn = try!(p.get());
            },
            None => unreachable!(),
        };
        Ok(String::from(""))
    }
}
