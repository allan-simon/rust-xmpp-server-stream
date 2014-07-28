// rust-xmpp
// Copyright (c) 2014 Florian Zeitz
//
// This project is MIT licensed.
// Please see the COPYING file for more information.

use std::io::net::tcp::TcpStream;
use std::io::BufferedStream;
use openssl::ssl::SslStream;
use std::io::IoResult;

use read_str::ReadString;

pub enum XmppSocket {
    Tcp(BufferedStream<TcpStream>),
    Tls(BufferedStream<SslStream<TcpStream>>),
    NoSock
}

impl Writer for XmppSocket {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        match *self {
            Tcp(ref mut stream) => stream.write(buf),
            Tls(ref mut stream) => stream.write(buf),
            NoSock => fail!("No socket yet")
        }
    }

    fn flush(&mut self) -> IoResult<()> {
        match *self {
            Tcp(ref mut stream) => stream.flush(),
            Tls(ref mut stream) => stream.flush(),
            NoSock => fail!("No socket yet")
        }
    }
}

impl ReadString for XmppSocket {
    fn read_str(&mut self) -> IoResult<String> {
        match *self {
            Tcp(ref mut stream) => stream.read_str(),
            Tls(ref mut stream) => stream.read_str(),
            NoSock => fail!("Tried to read string before socket exists")
        }
    }
}


