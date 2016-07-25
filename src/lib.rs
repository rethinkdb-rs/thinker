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
//! let mut conn = r.connect(ConnectOpts::default());
//! # }
//! ```

extern crate ql2;
extern crate reql;
extern crate r2d2;
extern crate byteorder;
extern crate bufstream;

pub mod conn;

use reql::*;
use conn::Connection;

pub struct Reql;

#[allow(non_upper_case_globals)]
pub const r: Reql = Reql;

impl R for Reql {
    type Connection = Connection;

    fn connect<T: IntoConnectOpts>(&self, opts: T) -> Self::Connection {
        conn::Connection::new(opts.into())
    }
}
