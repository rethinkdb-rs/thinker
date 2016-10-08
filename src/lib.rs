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
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
extern crate slog_term;

pub mod conn;

use reql::*;
use conn::{Connection, ConnectionManager};
use std::sync::RwLock;
use slog::DrainExt;

pub struct Reql {
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
    //type Connection = r2d2::Pool<ConnectionManager>;
    //type Error = r2d2::InitializationError;

    fn connect<T: IntoConnectOpts>(&self, opts: T) -> Result<()> {
        let config = r2d2::Config::builder()
            .pool_size(5)
            .build();
        debug!(r.logger, "Pool {:?}", config); 
        let manager = ConnectionManager::new(opts);

        match r2d2::Pool::new(config, manager).map_err(|e| {
            debug!(r.logger, "r2d2::Pool: {}", e);
            Error::Compile(CompileError{})
        }) {
            Ok(p) => { 
                debug!(r.logger, "{:?}", p); 
                let mut pool = r.pool.write().unwrap();
                *pool = Some(p);
                return Ok(());
            },
            Err(e) => {
                return Err(e)
            },
        };
    }
}
