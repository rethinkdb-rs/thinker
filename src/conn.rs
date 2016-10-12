//! RethinkDB Connection

use ql2::proto;
use std::net::TcpStream;
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};
use bufstream::BufStream;
use std::io::BufRead;
use std::str;
use r2d2;
use reql::*;
use super::session;
use super::serde_json;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

/// A connection to a RethinkDB database.
#[derive(Debug)]
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : TcpStream,
    auth     : String,
    token    : i64,
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

        let _ = try!(conn.handshake());
        Ok(conn)
    }

    fn handshake(&mut self) -> Result<()> {
        // Process: When you first open a connection, send the magic number
        // for the version of the protobuf you're targeting (in the [Version]
        // enum).  This should **NOT** be sent as a protobuf; just send the
        // little-endian 32-bit integer over the wire raw.  This number should
        // only be sent once per connection.
        let _ = try!(self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Version::V1_0 as u32));
        // The magic number shall be followed by an authorization key.  The
        // first 4 bytes are the length of the key to be sent as a little-endian
        // 32-bit integer, followed by the key string.  Even if there is no key,
        // an empty string should be sent (length 0 and no data).
        let _ = try!(self.stream.write_u32::<LittleEndian>(0));
        // Following the authorization key, the client shall send a magic number
        // for the communication protocol they want to use (in the [Protocol]
        // enum).  This shall be a little-endian 32-bit integer.
        let _ = try!(self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Protocol::JSON as u32));
        // Send request to server
        let _ = try!(self.stream.flush());

        // The server will then respond with a NULL-terminated string response.
        // "SUCCESS" indicates that the connection has been accepted. Any other
        // response indicates an error, and the response string should describe
        // the error.
        let mut resp = Vec::new();
        let null_str = b"\0"[0];
        let mut buf = BufStream::new(&self.stream);
        let _ = try!(buf.read_until(null_str, &mut resp));

        let _ = resp.pop();

        let cfg = try!(session.config.read().map_err(|err| {
            let msg = format!("failed to acquire read lock to the session config: {}", err);
            ConnectionError::PoolRead(msg)
        }));
        if resp.is_empty() {
            let msg = String::from("unable to connect for an unknown reason");
            crit!(cfg.logger, "{}", msg);
            return Err(From::from(ConnectionError::Other(msg)));
        };

        let resp = try!(str::from_utf8(&resp));
        // If it's not a JSON object it's an error
        if !resp.starts_with("{") {
            crit!(cfg.logger, "{}", resp);
            return Err(From::from(ConnectionError::Other(resp.to_string())));
        };
        let info: Info = match serde_json::from_str(&resp) {
            Ok(res) => res,
            Err(err) => {
                crit!(cfg.logger, "{}", err);
                return Err(From::from(err));
            },
        };
        debug!(cfg.logger, "{:?}", info);

        if !info.success {
            match info.error {
                Some(err) => return Err(From::from(ConnectionError::Other(err))),
                None => return Err(From::from(ConnectionError::Other(resp.to_string()))),
            };
        };

        Ok(())
    }
}

impl Connector for Connection {
    type Connection = Connection;
    // Close connection
    fn close(&self, _noreply_wait: bool) {}

    // Reconnect
    fn reconnect(&self, _noreply_wait: bool) -> Connection {
        unimplemented!();
    }

    // Use database
    fn use_db(&self, _db_name: &str) -> Connection {
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

    fn is_valid(&self, _conn: &mut Connection) -> Result<()> {
        Ok(())
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}
