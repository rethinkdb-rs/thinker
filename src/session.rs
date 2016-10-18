use std::sync::RwLock;
use slog::{DrainExt, Logger};
use conn::ConnectionManager;
use r2d2::Pool;
use slog_term;
use reql::*;
use std::error::Error as StdError;

lazy_static! {
    static ref POOL: RwLock<Option<Pool<ConnectionManager>>> = RwLock::new(None);

    static ref LOGGER: RwLock<Logger> = RwLock::new(
                    Logger::root(
                        slog_term::streamer().full().build().fuse(),
                        o!("version" => env!("CARGO_PKG_VERSION"))
                        )
                    );
}

pub struct Client;

impl Client {
    //pub fn logger() -> RwLock<Logger> {
    pub fn logger() -> &'static RwLock<Logger> {
        &LOGGER
    }

    //pub fn pool() -> RwLock<Option<Pool<ConnectionManager>>> {
    pub fn pool() -> &'static RwLock<Option<Pool<ConnectionManager>>> {
        &POOL
    }

    pub fn set_pool(p: Pool<ConnectionManager>) -> Result<()> {
        match POOL.write() {
            Ok(mut pool) => {
                *pool = Some(p);
                Ok(())
            },
            Err(err) => return Err(From::from(DriverError::Lock(err.description().to_string()))),
        }
    }
}
