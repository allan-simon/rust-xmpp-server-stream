// rust-xmpp
// Copyright (c) 2014 Florian Zeitz
// Copyright (c) 2014 Allan SIMON
//
// This project is MIT licensed.
// Please see the COPYING file for more information.

#![crate_name = "xmpp"]
#![crate_type = "lib"]

#![feature(macro_rules)]

extern crate serialize;

extern crate xml;
extern crate openssl;

use std::io::net::tcp::TcpStream;
use std::io::BufferedStream;
use std::io::IoResult;

use openssl::ssl::SslStream;


use read_str::ReadString;
use xmpp_send::XmppSend;

use std::io::net::tcp::TcpListener;
use std::io::{Listener, Acceptor};

mod read_str;
mod xmpp_send;
pub mod ns;

enum XmppSocket {
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

///
pub struct XmppServerListener {
    ip: String,
    port: u16
}

///
impl XmppServerListener {
    pub fn new(
        ip: &str,
        port: u16
    ) -> XmppServerListener {

        XmppServerListener {
            ip: ip.to_string(),
            port: port
        }
    }

    pub fn listen(&mut self) {
        let listener = TcpListener::bind(
            self.ip.as_slice(),
            self.port
        );
        let mut acceptor= listener.listen().unwrap();
        for opt_stream in acceptor.incoming() {
            spawn(proc() {
                let mut xmppStream = XmppServerStream::new(opt_stream.unwrap());
                xmppStream.handle();


            })
        }
    }

}

///
///
///
struct XmppServerHandler {
    socket: XmppSocket
}


///
///
///
impl XmppServerHandler {
    fn start_stream(&mut self, domain: &str) -> IoResult<()> {
        let start = format!(
            "<?xml version='1.0'?>\n\
            <stream:stream \
                xmlns:stream='{}'
                xmlns='{}' \
                version='1.0' \
                from='{}'\
                id='{}'
            >",
            ns::STREAMS,
            ns::JABBER_CLIENT,
            domain,
            "arandomid"
        );
        self.send(start)
    }

    fn close_stream(&mut self) -> IoResult<()> {
        self.send("</stream:stream>")
    }

    fn send<T: XmppSend>(&mut self, data: T) -> IoResult<()> {
        let data = data.xmpp_str();
        println!("Out: {}", data);
        try!(self.socket.write(data.as_slice().as_bytes()));
        self.socket.flush()
    }
}


/// Represent a server side XMPP stream
pub struct XmppServerStream {
    parser: xml::Parser,
    builder: xml::ElementBuilder,
    handler: XmppServerHandler,
}


impl XmppServerStream {

    pub fn new(
        stream: TcpStream
    ) -> XmppServerStream {
        XmppServerStream {
            parser: xml::Parser::new(),
            builder: xml::ElementBuilder::new(),
            handler: XmppServerHandler {
                socket: Tcp(BufferedStream::new(stream))
            }
        }
    }

    pub fn handle(&mut self) {

        loop {
            let string = {
                let socket = &mut self.handler.socket;
                match socket.read_str() {
                    Ok(string) => string,
                    Err(e) => return
                }
            };
            println!("raw string {}", string);
            self.parser.feed_str(string.as_slice());

            let builder = &mut self.builder;
            let handler = &mut self.handler;

            for event in self.parser { match event {
                Ok(xml::StartTag(xml::StartTag {
                    name: ref name,
                    ns: Some(ref ns),
                    prefix: ref prefix, ..
                })) if name.as_slice() == "stream" && ns.as_slice() == ns::STREAMS => {
                    println!("In: Stream start");
                    match *prefix {
                        Some(ref prefix) => {
                            *builder = xml::ElementBuilder::new();
                            builder.set_default_ns(ns::JABBER_CLIENT);
                            builder.define_prefix(prefix.as_slice(), ns::STREAMS);
                            handler.start_stream("localhost".as_slice());
                        },
                        None => ()
                    }
                },
                Ok(xml::EndTag(xml::EndTag {
                    name: ref name,
                    ns: Some(ref ns), ..
                })) if name.as_slice() == "stream" && ns.as_slice() == ns::STREAMS => {
                    println!("In: Stream end");
                    handler.close_stream();
                    break;
                }
               Ok(event) => {
                    match builder.push_event(event) {
                        Ok(Some(ref e)) => {
                            println!("In: {}", e)
                        }
                        Ok(None) => (),
                        Err(e) => println!("{}", e),
                    }
                },
                Err(e) => println!(
                    "Line: {} Column: {} Msg: {}",
                    e.line,
                    e.col, e.msg),
            };}
        }
        println!("we close");
    }

}
