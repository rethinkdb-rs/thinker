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
//! r.connect(ConnectOpts::default());
//! # }
//! ```

#![allow(non_upper_case_globals)]

extern crate ql2;
extern crate reql;
extern crate r2d2;
extern crate byteorder;
extern crate bufstream;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate slog;
extern crate slog_term;

pub mod conn;

use reql::*;
use conn::{Connection, ConnectionManager};
use std::sync::RwLock;
use slog::DrainExt;

pub struct Reql{
    pub pool: RwLock<Option<r2d2::Pool<ConnectionManager>>>,
    pub logger: slog::Logger,
}

lazy_static! {
    pub static ref r: Reql = Reql{
        pool: RwLock::new(None),
        logger: slog::Logger::root(slog_term::streamer().full().build().fuse(), o!("version" => env!("CARGO_PKG_VERSION"))),
    };
}

impl R for Reql {
    /// Creates a connection pool
    fn connect<T: IntoConnectOpts>(&self, opts: T) -> Result<()> {
        info!(r.logger, "Trying to create a connection pool...");
        // Configure the `r2d2` connection pool
        let config = r2d2::Config::default();
        // Create a connection pool
        let manager = ConnectionManager::new(opts);
        let p = try!(r2d2::Pool::new(config, manager));
        // and save it into `Reql::pool` which is globally acessible from easier access anywhere.
        let mut pool = try!(r.pool.write().map_err(|err| {
            let msg = format!("failed to acquire write lock to the connection pool: {}", err);
            crit!(r.logger, "{}", msg);
            ConnectionError::PoolWrite(msg)
        }));
        *pool = Some(p);
        Ok(())
    }
}
