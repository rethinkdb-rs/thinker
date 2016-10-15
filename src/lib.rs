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
extern crate protobuf;

pub mod conn;
pub mod types;

use reql::*;
use conn::ConnectionManager;
use std::sync::RwLock;
use slog::DrainExt;
use r2d2::{Pool, Config as PoolConfig};
use ql2::proto;
use protobuf::repeated::RepeatedField;

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

pub struct RootTerm(Result<proto::Term>);

impl Reql {
    pub fn table(&self, name: &str) -> RootTerm {
        // Datum
        let mut datum = proto::Datum::new();
        datum.set_field_type(proto::Datum_DatumType::R_STR);
        datum.set_r_str(name.to_string());
        // Args
        let mut args = proto::Term::new();
        args.set_field_type(proto::Term_TermType::DATUM);
        args.set_datum(datum);
        // Table
        let mut table = proto::Term::new();
        table.set_field_type(proto::Term_TermType::TABLE);
        table.set_args(RepeatedField::from_vec(vec![args]));
        RootTerm(Ok(table))
    }
}

impl RootTerm {
    pub fn run(self) -> Result<String> {
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
        if let Some(ref p) = *pool {
            let _conn = try!(p.get());
        };
        Ok(String::new())
    }
}
