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

use reql::*;
use conn::ConnectionManager;
use std::sync::RwLock;
use slog::DrainExt;

pub struct Reql{
    pub config: RwLock<ReqlConfig>,
}

#[derive(Debug)]
pub struct ReqlConfig{
    pub pool: Option<r2d2::Pool<ConnectionManager>>,
    pub logger: slog::Logger,
}

lazy_static! {
    pub static ref r: Reql = Reql{
        config: RwLock::new(ReqlConfig{
            pool: None,
            logger: slog::Logger::root(
                slog_term::streamer().full().build().fuse(),
                o!("version" => env!("CARGO_PKG_VERSION"))
                ),
        }),
    };
}

impl R for Reql {
    /// Creates a connection pool
    fn connect<T: IntoConnectOpts>(&self, opts: T) -> Result<()> {
        if r.config.read().unwrap().pool.is_some() {
            // Pool is already set
            return Ok(());
        }
        info!(r.config.read().unwrap().logger, "Trying to create a connection pool...");
        // Configure the `r2d2` connection pool
        let config = r2d2::Config::default();
        // Create a connection pool
        let manager = ConnectionManager::new(opts);
        let p = try!(r2d2::Pool::new(config, manager));
        // and save it into `Reql::pool` which is globally acessible from easier access anywhere.
        let mut reql = r.config.write().unwrap();
        reql.pool = Some(p);
        Ok(())
    }
}
