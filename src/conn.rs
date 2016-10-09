//! RethinkDB Connection

#![allow(dead_code)]
#![allow(unused_variables)]

extern crate serde;
extern crate serde_json;

use ql2::proto;
use std::net::TcpStream;
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};
use bufstream::BufStream;
use std::io::BufRead;
use std::str;
use r2d2;
use reql::*;
use super::r;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

/// A connection to a RethinkDB database.
#[derive(Debug)]
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : TcpStream,
    auth     : String,
    token    : usize,
}

impl Connection {
    pub fn new(opts: ConnectOpts) -> Result<Connection> {
        let stream = try!(TcpStream::connect((opts.host, opts.port)));

        let mut conn = Connection{
            host    : opts.host.to_string(),
            port    : opts.port,
            stream  : stream,
            auth    : "AUTH".to_string(),
            // @TODO: implement proper token generation
            token   : 1,
        };

        conn.handshake();
        Ok(conn)
    }

    fn handshake(&mut self)  {
        self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Version::V1_0 as u32);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Protocol::JSON as u32);
        self.stream.flush();

        let mut resp = Vec::new();
        let null_str = b"\0"[0];
        let mut buf = BufStream::new(&self.stream);
        buf.read_until(null_str, &mut resp);

        let _ = resp.pop();

        if resp.is_empty() {
            debug!(r.logger, "Unable to connect");
        } else {
            let resp = str::from_utf8(&resp).unwrap();
            let info: Info = serde_json::from_str(&resp).unwrap();
            debug!(r.logger, "{:?}", info);
        }
    }
}

impl Connector for Connection {
    type Connection = Connection;
    // Close connection
    fn close(&self, noreply_wait: bool) {}

    // Reconnect
    fn reconnect(&self, noreply_wait: bool) -> Connection {
        unimplemented!();
    }

    // Use database
    fn use_db(&self, db_name: &str) -> Connection {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct ConnectionManager {
    opts: ConnectOpts,
}

impl ConnectionManager {
    pub fn new<T: IntoConnectOpts>(opts: T) -> ConnectionManager {
        ConnectionManager {
            opts: opts.into(),
        }
    }
}

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Connection> {
        Connection::new(self.opts.clone())
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<()> {
        Ok(())
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}
