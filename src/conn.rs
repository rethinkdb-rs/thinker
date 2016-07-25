//! RethinkDB Connection

#![allow(dead_code)]
#![allow(unused_variables)]

use ql2::proto;
use std::net::TcpStream;
use std::io::Write;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use bufstream::BufStream;
use std::io::BufRead;
use std::io;
use r2d2;
use reql::*;


/// A connection to a RethinkDB database.
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : TcpStream,
    auth     : String,
    token    : usize,
}

impl Connection {
    pub fn new(opts: ConnectOpts) -> Connection {
        let stream = TcpStream::connect((opts.host, opts.port)).ok().unwrap();

        let mut conn = Connection{
            host    : opts.host.to_string(),
            port    : opts.port,
            stream  : stream,
            auth    : "AUTH".to_string(),
            token   : 1,
        };

        conn.handshake();
        conn
    }

    fn handshake(&mut self)  {
        self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Version::V1_0 as u32);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Protocol::JSON as u32);
        self.stream.flush();

        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        let mut buf = BufStream::new(&self.stream);
        buf.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => print!("{:?}", "OK, foi"),
            _ => print!("{:?}", "Unable to connect")
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

struct ConnectionManager;

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = TimeoutError;

    fn connect(&self) -> Result<Connection, TimeoutError> {
        unimplemented!();
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<(), TimeoutError> {
        unimplemented!();
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}
