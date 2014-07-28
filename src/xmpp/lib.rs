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

use server_stream::XmppServerStream;

use std::io::net::tcp::TcpListener;
use std::io::{Listener, Acceptor};

mod read_str;
mod xmpp_send;
mod xmpp_socket;
mod server_stream;
mod server_handler;
pub mod ns;

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
                let mut xmppStream = XmppServerStream::new(
                    opt_stream.unwrap()
                );
                xmppStream.handle();

            })
        }
    }

}

