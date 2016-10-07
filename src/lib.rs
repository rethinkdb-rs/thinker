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
//! let pool = r.connect(ConnectOpts::default()).unwrap();
//! let conn = pool.get().unwrap();
//! # }
//! ```

extern crate ql2;
extern crate reql;
extern crate r2d2;
extern crate byteorder;
extern crate bufstream;

pub mod conn;

use reql::*;
use conn::{Connection, ConnectionManager};

pub struct Reql;

#[allow(non_upper_case_globals)]
pub const r: Reql = Reql;

impl R for Reql {
    type Connection = r2d2::Pool<ConnectionManager>;
    //type Error = r2d2::InitializationError;

    fn connect<T: IntoConnectOpts>(&self, opts: T) -> Result<Self::Connection> {
        let config = r2d2::Config::builder()
            .pool_size(5)
            .build();
        let manager = ConnectionManager::new(opts);

        r2d2::Pool::new(config, manager).map_err(|_| Error::Compile(CompileError{}))
    }
}
