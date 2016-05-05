//! RethinkDB Driver
//!
//! ```rust
//! extern crate thinker;
//!
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

use conn::{Connection, Opts};

pub struct Reql;

#[allow(non_upper_case_globals)]
pub const r: Reql = Reql;

impl Reql {
    pub fn connect(&self, opts: Opts) -> Connection {
        Connection::new(opts)
    }
}
