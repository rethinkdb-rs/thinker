//! RethinkDB Driver
//!
//! ```rust
//! extern crate reql;
//! extern crate thinker;
//!
//! use reql::R;
//! use thinker::r;
//!
//! # fn main() {
//! let mut conn = r.connect(Default::default());
//! # }
//! ```

extern crate ql2;
extern crate reql;
extern crate r2d2;
extern crate byteorder;
extern crate bufstream;

pub mod conn;

use conn::Connection;

pub struct R;

#[allow(non_upper_case_globals)]
pub const r: R = R;

impl reql::R for R {
    type Connection = Connection;
    fn connect(&self, opts: reql::conn::Opts) -> Self::Connection {
        conn::Connection::new(opts)
    }
}
