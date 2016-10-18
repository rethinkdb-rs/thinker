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
extern crate scram;

pub mod conn;
pub mod types;

use reql::*;
use conn::ConnectionManager;
use std::sync::RwLock;
use slog::DrainExt;
use r2d2::{Pool, Config as PoolConfig};
use ql2::proto;
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};
use std::io::Read;
use std::str;
use protobuf::ProtobufEnum;
use byteorder::ReadBytesExt;

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

pub struct RootCommand(Result<String>);
struct Command;
struct Query;

impl Reql {
    pub fn db(&self, name: &str) -> RootCommand {
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::DB,
                    format!("{:?}", name),
                    None,
                    None
                    )))
    }

    pub fn table(&self, name: &str) -> RootCommand {
        r.db("test").table(name)
    }

    pub fn object(&self) -> serde_json::builder::ObjectBuilder {
        serde_json::builder::ObjectBuilder::new()
    }

    pub fn array(&self) -> serde_json::builder::ArrayBuilder {
        serde_json::builder::ArrayBuilder::new()
    }
}

impl RootCommand {
    pub fn table(self, name: &str) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::TABLE,
                    format!("{:?}", name),
                    None,
                    Some(commands)
                    )))
    }

    pub fn insert(self, expr: serde_json::Value) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        let data = match serde_json::to_string(&expr) {
            Ok(f) => f,
            Err(e) => return RootCommand(Err(From::from(DriverError::Json(e)))),
        };
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::INSERT,
                    data,
                    None,
                    Some(commands),
                    )))
    }

    pub fn filter(self, expr: serde_json::Value) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        let filter = match serde_json::to_string(&expr) {
            Ok(f) => f,
            Err(e) => return RootCommand(Err(From::from(DriverError::Json(e)))),
        };
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::FILTER,
                    filter,
                    None,
                    Some(commands),
                    )))
    }

    pub fn run(self) -> Result<String> {
        let commands = try!(self.0);
        let ref cfg = try!(session.config.read().map_err(|err| {
            let msg = format!("failed to acquire read lock to the session config: {}", err);
            ConnectionError::PoolRead(msg)
        }));
        if let Some(ref p) = cfg.pool {
            let mut conn = try!(p.get());
            conn.token += 1;
            let query = Query::wrap(
                proto::Query_QueryType::START,
                Some(commands),
                None);
            debug!(cfg.logger, "{}", query);
            let query = query.as_bytes();
            let token = conn.token;
            let _ = try!(conn.stream.write_u64::<LittleEndian>(token));
            let _ = try!(conn.stream.write_u32::<LittleEndian>(query.len() as u32));
            let _ = try!(conn.stream.write_all(query));
            let _ = try!(conn.stream.flush());

            // @TODO use response_token to implement parallel reads and writes?
            // let response_token = try!(conn.stream.read_u64::<LittleEndian>());
            let _ = try!(conn.stream.read_u64::<LittleEndian>());
            let len = try!(conn.stream.read_u32::<LittleEndian>());

            let mut resp = vec![0u8; len as usize];
            try!(conn.stream.read_exact(&mut resp));
            let resp = try!(str::from_utf8(&resp));
            debug!(cfg.logger, "{}", resp);
        } else {
            let msg = String::from("Your connection pool is not initialised. \
                                   Use `r.connection().connect()` to initialise the pool \
                                   before trying to send any connections to the database. \
                                   This is typically done in the `main` function.");
            return Err(From::from(ConnectionError::Other(msg)));
        }
        Ok(String::new())
    }
}

impl Command {
    fn wrap(command: proto::Term_TermType, arguments: String, options: Option<String>, commands: Option<String>) -> String {
        let mut cmds = format!("[{},", command.value());
        let args: String;
        if let Some(commands) = commands {
            args = format!("{},{}", commands, arguments);
        } else {
            args = arguments;
        }
        cmds.push_str(format!("[{}]", args).as_str());
        if let Some(options) = options {
            cmds.push_str(format!(",{{{}}}", options).as_str());
        }
        cmds.push(']');
        cmds
    }
}

impl Query {
    fn wrap(query_type: proto::Query_QueryType, query: Option<String>, options: Option<String>) -> String {
        let mut qry = format!("[{}", query_type.value());
        if let Some(query) = query {
            qry.push_str(format!(",{}", query).as_str());
        }
        if let Some(options) = options {
            qry.push_str(format!(",{}", options).as_str());
        }
        qry.push_str("]");
        qry
    }
}
