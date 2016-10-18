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
//! //r.connect(ConnectOpts::default()).unwrap();
//! # }
//! ```

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
extern crate scram;

pub mod conn;
pub mod types;
pub mod commands;
pub mod session;

#[allow(non_upper_case_globals)]
pub const r: session::Client = session::Client;
